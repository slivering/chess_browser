/// Implement FEN, legal moves, result for Board.


use crate::position::*;
use crate::prelude::*;
use crate::units::Direction;
use crate::bit;
use crate::moves::{PGNMove, CheckType, castling};
use crate::movegen::{MoveGen, MoveGenMasked, MoveGenerator};
use crate::game::{GameResult, WinType, DrawType};


impl Board {

    /// Builds a Board from the FEN notation.
    /// ```
    /// use chess_std::{Board};
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen).unwrap();
    /// assert_eq!(board, Board::new());
    /// ```
    #[cfg(feature = "fen")]
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let items: Vec<_> = fen.split_whitespace().collect();
        if items.len() != 6 {
            return Err("Not enough fields".to_owned());
        }

        let mut board = Board::default();
        let board_data = items[0];
        let mut r = Rank(8);
        for row in board_data.split('/') {
            r.0 -= 1;
            let mut f = File::A;
            for c in row.chars() {
                if c.is_digit(9) {
                    f.0 += c as u8 - b'0';
                } else {
                    let sq = Square::new(r, f);
                    let pc = Piece::try_from(c)?;
                    board.add_piece(pc, sq);
                    f.0 += 1;
                }
            }
        }
        let turn_char = items[1].as_bytes()[0] as char;
        board.turn = Color::try_from(turn_char)?;
        board.update_attacks();
        board.rights = [castling::NO_RIGHTS; NUM_PLAYERS];
        for right in items[2].chars() {
            match right {
                'K' => board.add_right(White, Side::King),
                'Q' => board.add_right(White, Side::Queen),
                'k' => board.add_right(Black, Side::King),
                'q' => board.add_right(Black, Side::Queen),
                '-' => break,
                _   => {
                    return Err("Couldn't parse castling right".to_owned());
                }
            }
        }
        let sq_data = items[3];
        board.ep_target = if sq_data == "-" {
            None
        } else {
            Some(Square::from_san(sq_data)?)
        };
        board.half_move_clock = items[4].parse().unwrap_or(1);
        board.last_cap_or_push = board.half_move_clock*2;
        Ok(board)
    }

    /// Returns the positional FEN notation of this `Board`.
    ///
    /// ```
    /// use chess_std::Board;
    /// println!("{}", Board::new().to_fen());
    /// ```
    #[cfg(feature = "fen")]
    pub fn to_fen(&self) -> String {
        let mut s = String::new();
        // Board
        for r in (Rank::R1..=Rank::R8).rev() {
            let mut num_empty = 0;
            for f in File::A..=File::H {
                if let Some(pc) = self.piece_at(Square::new(r, f)) {
                    if num_empty > 0 {
                        s.push_str(&num_empty.to_string());
                        num_empty = 0;
                    }
                    s.push(pc.to_char());
                } else {
                    num_empty += 1;
                }
            }
            if num_empty > 0 {
                s.push_str(&num_empty.to_string());
            }
            if r != Rank::R1 {
                s.push('/');
            }
        }
        // Turn
        s.push_str(&format!(" {} ", &self.turn.to_string()));
        // Castling rights
        if self.rights == NO_PLAYERS_RIGHTS {
            s.push('-');
        } else {
            for player in &PLAYERS {
                if !self.has_right(*player, Side::King) {
                    let pc = Piece{ color: *player, ptype: King };
                    s.push(pc.to_char());
                }
                if !self.has_right(*player, Side::Queen) {
                    let pc = Piece{ color: *player, ptype: Queen };
                    s.push(pc.to_char());
                }
            }
        }
        // En passant target + clocks
        s.push_str(&format!(
            " {} {} {}",
            if self.ep_target.is_some() {
                self.ep_target.unwrap().san()
            } else {
                "-".to_owned()
            },
            self.half_move_clock,
            self.num_moves_played()
        )[..]);
        s
    }

    /// Extend a plain move with additional data as a PGN move.
    /// Keep in mind that this function is slow.
    #[cfg(feature = "pgn")]
    pub fn pgn_move(&self, mv: Move) -> PGNMove {
        use CheckType::*;
        let next_board = self.play_move(mv);
        PGNMove::from_plain(
            mv,
            self.type_moved_by(mv),
            self.captured_by(mv).map(|pc| pc.ptype),
            if next_board.in_checkmate() {
                Checkmate
            } else if next_board.in_check() {
                Check
            } else {
                None
            }
        )
    }

    /// Returns a generator over the legal moves.
    pub fn legal_moves(&self) -> MoveGen {
        MoveGen::new_from(self)
    }

    /// Returns an generator over the legal moves from a square,
    /// using `Board::legal_moves()`.
    pub fn legal_moves_from(&self, sq: Square) -> MoveGenMasked {
        let mut gen = MoveGenMasked::from(self.legal_moves());
        gen.set_origin_mask(bit::single(sq));
        gen
    }

    /// Returns a masked generator over the capturing moves,
    /// using `Board::legal_moves()`.
    pub fn legal_captures(&self) -> MoveGenMasked {
        let mut gen = MoveGenMasked::from(self.legal_moves());
        gen.set_destination_mask(self.opponent_color());
        gen
    }

    /// Returns a masked generator over the legal moves of a piece,
    /// using `Board::legal_moves()`.
    pub fn legal_moves_of(&self, ptype: PieceType) -> MoveGenMasked {
        let mut gen = MoveGenMasked::from(self.legal_moves());
        gen.set_origin_mask(self.own_piece_type(ptype));
        gen
    }

    /// Whether a move can be played, using `Board::legal_moves()`.
    pub fn is_move_legal(&self, mv: Move) -> bool {
        self.legal_moves().contains(mv)
    }

    /// The number of legal moves, using `Board::legal_moves()`.
    /// Promotions are counted for each piece.
    /// 
    /// Keep in mind this does recompute the move generator.
    /// Use the `len()` method on `Board.legal_moves()` for efficiency.
    pub fn num_moves(&self) -> usize {
        self.legal_moves().len()
    }

    /// Apply the move in place. This assumes the move is legal.
    ///
    /// # Panics
    /// 
    /// When the move selection, capture or flag detail is invalid.
    pub fn apply_move(&mut self, mv: Move) {
        use MoveFlag::*;
        if mv.is_none() {
            return
        }
        self.update_meta_with(mv);

        let moved = self.piece_at(mv.from).expect("Must move a piece");
        assert_eq!(self.color_at(mv.from), Some(self.turn),
                "Cannot select a piece which color is not the turn");
        if let Some(cap) = self.piece_at(mv.to) {
            assert_ne!(cap.color, self.turn, "Cannot capture a friend piece");
            self.remove_piece(cap, mv.to);
        }
        self.move_piece(moved, mv.from, mv.to);
        match mv.flag {
            Quiet => {},
            EnPassant(pawn_sq) => {
                let pawn = Piece{ color: self.turn.opponent(), ptype: Pawn };
                assert_eq!(Some(pawn), self.piece_at(pawn_sq),
                           "Illegal en passant of a non-pawn piece: {}", pawn);
                self.remove_piece(pawn, pawn_sq);
            }
            Promotion(new) => {
                assert_eq!(moved.ptype, Pawn, "Cannot promote {}", moved);
                assert!(new.can_be_promotion(), "Cannot promote into {}", new);
                self.remove_piece(moved, mv.to);
                self.add_piece(Piece{ color: self.turn, ptype: new }, mv.to);
            }
            Castling(side) => {
                // get the `half` moves according to the turn and the side.
                if let King = moved.ptype {
                    let (rfrom, rto) = Move::rook_castling_coords(self.turn, side);
                    self.move_piece(Piece{ color: self.turn, ptype: Rook }, rfrom, rto);
                } else {
                    panic!("Cannot castle with {:?}", moved);
                }
            }
        }
        if self.turn == Black {
            self.half_move_clock += 1;
        }
        self.turn = self.turn.opponent();
        self.update_attacks();        
    }

    /// Returns the subsequent board after applying the move.
    ///
    /// ```
    /// use chess_std::{Square, Board};
    ///
    /// let mut board = Board::new();
    /// let mv = board.legal_moves_from(Square::D2).next().unwrap();
    /// board = board.play_move(mv);
    /// ```
    pub fn play_move(&self, mv: Move) -> Self {
        let mut next_board = self.clone();
        next_board.apply_move(mv);
        next_board
    }

    // Update the castling rights, the en passant target and the last capture/push
    // according to a move that's going to be played.
    #[inline]
    fn update_meta_with(&mut self, mv: Move) {
        fn remove_right_for(board: &mut Board, sq: Square) {
            match sq {
                Square::H1 => board.remove_right(White, Side::King),
                Square::E1 => board.remove_rights(White),
                Square::A1 => board.remove_right(White, Side::Queen),
                Square::H8 => board.remove_right(Black, Side::King),
                Square::E8 => board.remove_rights(Black),
                Square::A8 => board.remove_right(Black, Side::Queen),
                _          => {}
            };
        }
        remove_right_for(self, mv.from);
        remove_right_for(self, mv.to);
        let moved = self.type_moved_by(mv);
        self.ep_target = None;
        if mv.is_double_push(self.turn) && moved == Pawn {
            self.ep_target = Some(mv.from.shift(Direction::of_pawns(self.turn)));
        };

        if self.captured_by(mv).is_some() || moved == Pawn {
           self.last_cap_or_push = self.num_moves_played();
        }
    }


    /// Whether the current player's king is checked.
    #[inline]
    pub fn in_check(&self) -> bool {
        self.checkers.is_populated()
    }

    /// Whether the current player's king is checkmated.
    /// 
    /// This does recompute the number of legal moves.
    #[inline]
    pub fn in_checkmate(&self) -> bool {
        self.is_finished() && self.in_check()
    }

    /// Whether the current player's king is stuck in stalemate.
    /// 
    /// This does recompute the number of legal moves.
    #[inline]
    pub fn in_stalemate(&self) -> bool {
        self.is_finished() && !self.in_check()
    }

    /// Whether the result is checkmate or stalemate.
    /// 
    /// This does recompute the number of legal moves.
    pub fn is_finished(&self) -> bool {
        self.num_moves() == 0
    }

    /// A theorical evaluation whether there aren't enough pieces to win.
    /// 
    /// ```
    /// use chess_std::prelude::*;
    /// use chess_std::board::Builder;
    /// 
    /// let board = Builder::new()
    ///     .piece(W_KING, Square::D3)
    ///     .piece(B_KING, Square::F6)
    ///     .piece(W_BISHOP, Square::B7)
    ///     .build().unwrap();
    /// 
    /// assert!(board.is_material_insufficient());
    /// ```
    pub fn is_material_insufficient(&self) -> bool {
        match self.occupied().pop_count() {
            2 => true, // King vs King
            3 => {
                self.piece_type(Knight).pop_count() == 1 ||
                self.piece_type(Bishop).pop_count() == 1
            },
            4 => {
                let w_b = self.of_color_and_type(White, Bishop);
                let b_b = self.of_color_and_type(Black, Bishop);
                // Only two bishops on squares of the ours color
                w_b.pop_count() == 1 && b_b.pop_count() == 1 &&
                w_b.scan_forward().is_dark() == b_b.scan_forward().is_dark()
            }
            _ => false
        }
    }

    /// Whether a draw type can be claimed, except ThreefoldRepetition.
    pub fn can_claim_draw_with(&self, dt: DrawType) -> bool {
        use DrawType::*;
        match dt {
            Agreement => true,
            FiftyMoveRule => self.num_moves_played() - self.last_cap_or_push > 50,
            InsufficientMaterial => self.is_material_insufficient(),
            Stalemate => false, // Cannot claim stalemate
            ThreefoldRepetition => false // Don't handle this
        }
    }

    /// When `self.get_result() == GameResult::NoResult`,
    /// however fifty-move rule or insufficient material might be claimed.
    ///
    /// NOTE: use Game::can_claim_draw for threefold repetition.
    pub fn can_claim_draw(&self) -> bool {
        use DrawType::*;
        self.can_claim_draw_with(FiftyMoveRule) ||
        self.can_claim_draw_with(InsufficientMaterial)
    }

    /// Either the game is still ongoing, or a result (win or draw) can be declared.
    /// 
    /// This does recompute the number of legal moves.
    pub fn get_result(&self) -> GameResult {
        use {GameResult::*, WinType::*, DrawType::*};
        if self.is_finished() {
            if self.in_check() {
                Win(self.turn.opponent(), Checkmate)
            } else {
                Draw(Stalemate)
            } 
        } else if self.can_claim_draw_with(FiftyMoveRule) {
            Draw(FiftyMoveRule)
        } else if self.can_claim_draw_with(InsufficientMaterial) {
            Draw(InsufficientMaterial)
        } else {
            NoResult
        }
    }
}



use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, ft: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(ft, "  a b c d e f g h")?;
        for r in (Rank::R1..=Rank::R8).rev() {
            write!(ft, "\n{}", r.to_char())?;
            for f in File::A..=File::H {
                let at = self.piece_at(Square::new(r, f));
                write!(ft, " {}", if let Some(pc) = at {
                    pc.to_char()
                } else {
                    '-'
                })?;
            }
        }
        write!(ft, "\nTurn: {:?}\t", self.turn)?;
        write!(ft, "Halfmove clock: {}\t", self.half_move_clock)?;
        Ok(())
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, ft: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(ft, "{}", self.to_fen())
    }
}