/* The `BoardBuilder`structure.*/

use crate::position::*;
use crate::moves::castling;
use crate::bit;
use crate::prelude::*;


/// A board builder.
/// 
/// Useful to setup a `Board` from a custom position.
/// 
/// ```
/// use chess_std::prelude::*;
/// use chess_std::board::{Board, Builder};
/// 
/// let board = Builder::new()
///     .piece(W_KING, Square::A2)
///     .piece(B_PAWN, Square::C2)
///     .piece(B_KING, Square::B4)
///     .turn(Color::Black)
///     .half_move_clock(0)
///     .build().unwrap();
/// 
/// assert_eq!(board, Board::from_fen("8/8/8/8/1k6/8/K1p5/8 b - - 0 1").unwrap());
/// ```
pub struct Builder {
    pieces: Pieces,
    colors: Colors,
    turn: Color,
    hash: zobrist::Hash,

    half_move_clock: u32,
    rights: PlayersRights
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Board> for Builder {
    fn from(board: Board) -> Self {
        Self {
            pieces: board.pieces,
            colors: board.colors,
            turn: board.turn,
            hash: board.hash,
            half_move_clock: board.half_move_clock,
            rights: ALL_PLAYERS_RIGHTS
        }
    }
}

impl Builder {
    /// Start with an empty position.
    pub fn new() -> Self {
        Self {
            pieces: [bit::EMPTY; NUM_PIECE_TYPES],
            colors: [bit::EMPTY; NUM_PLAYERS],
            turn: White,
            hash: zobrist::INITIAL_HASH,
            half_move_clock: 0,
            rights: NO_PLAYERS_RIGHTS
        }
    }

    /// Add a piece at a square.
    pub fn piece(&mut self, pc: Piece, sq: Square) -> &mut Self {
        if !self.pieces[pc.ptype.index()].get(sq) {
            self.pieces[pc.ptype.index()].add(sq);
            self.colors[pc.color.index()].add(sq);
            self.hash ^= zobrist::hash_piece(pc, sq);
        }
        self
    }

    /// Set the turn.
    pub fn turn(&mut self, col: Color) -> &mut Self {
        self.turn = col;
        self
    }

    /// Set the half-move clock.
    pub fn half_move_clock(&mut self, hmc: u32) -> &mut Self {
        self.half_move_clock = hmc;
        self
    }

    /// Set a castling right for a player and a side.
    pub fn castling_right(&mut self, player: Color, side: castling::Side) -> &mut Self {
        self.rights[player.index()][side.index()] = true;
        self
    }

    /// Returns `Some` if the board is valid, else `None`.
    pub fn build(&self) -> Option<Board> {
        let mut board = Board {
            pieces: self.pieces,
            colors: self.colors,
            hash: self.hash,
            turn: self.turn,

            half_move_clock: self.half_move_clock,
            ep_target: None,
            rights: self.rights,
            last_cap_or_push: self.half_move_clock * 2,

            checkers: bit::EMPTY,
            pinned: bit::EMPTY,
        };
        if !board.is_valid() {
            return None;
        }
        board.rehash();
        Some(board)
    }
}