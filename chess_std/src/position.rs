/* The implementation of `Board`. */


use crate::{bit, Bitboard};
use crate::prelude::*;
use crate::moves::castling;


// Each piece is associated to a bitboard.
pub(crate) type Pieces = [Bitboard; NUM_PIECE_TYPES];

// Each player has a bitboard of pieces.
pub(crate) type Colors = [Bitboard; NUM_PLAYERS];

// The rights for both players.
pub (crate) type PlayersRights = [castling::Rights; NUM_PLAYERS];

pub(crate) const ALL_PLAYERS_RIGHTS: PlayersRights = [castling::ALL_RIGHTS; NUM_PLAYERS];
pub(crate) const NO_PLAYERS_RIGHTS:  PlayersRights = [castling::NO_RIGHTS; NUM_PLAYERS];

const INITIAL_GRID: Pieces = [
    Bitboard(bit::RANK_2.0 | bit::RANK_7.0),
    Bitboard(0b0100_0010 | 0b0100_0010 << 56),
    Bitboard(0b0010_0100 | 0b0010_0100 << 56),
    Bitboard(0b1000_0001 | 0b1000_0001 << 56),
    Bitboard(0b0000_1000 | 0b0000_1000 << 56),
    Bitboard(0b0001_0000 | 0b0001_0000 << 56),
];

const INITIAL_COLORS: Colors = [
    Bitboard(bit::RANK_1.0 | bit::RANK_2.0),
    Bitboard(bit::RANK_7.0 | bit::RANK_8.0),
];

/// This is the method used by Board to give a collision-safe hash.
pub mod zobrist {
    use crate::units::*;
    use super::PlayersRights;
    pub type Hash = u64;
    include!("./generate/zobrist_tables.rs");

    pub fn hash_piece(pc: Piece, sq: Square) -> Hash {
        HASH_PIECE[pc.index() + NUM_PIECES * sq.index()]
    }

    pub fn hash_square(sq: Square) -> Hash {
        HASH_SQUARE[sq.index()]
    }

    pub fn hash_color(col: Color) -> Hash {
        HASH_COLOR[col.index()]
    }

    pub fn hash_rights(rights: PlayersRights) -> Hash {
        let a = rights[0][0] as usize;
        let b = rights[0][1] as usize;
        let c = rights[1][0] as usize;
        let d = rights[1][1] as usize;
        HASH_RIGHTS[a + (b << 1) + (c << 2) + (d << 3)]
    }
}



/// A `Board` is a representation of the game that views, modifies the position.
/// It can generate legal moves and can apply them on a successor.
/// 
/// Use this instead of Game for performance, if knowing the previous boards
/// and moves is not needed.
/// 
/// The `Board` it is internally implemented by `Bitboard` arrays.
#[derive(Clone)]
pub struct Board {
    pub(crate) pieces: Pieces,
    pub(crate) colors: Colors,
    pub(crate) hash: zobrist::Hash,              // Positional hash
    pub turn: Color,

    pub half_move_clock: u32,
    pub(crate) ep_target: Option<Square>,
    pub(crate) rights: PlayersRights,
    pub(crate) last_cap_or_push: u32,            // As a move index

    pub(crate) checkers: Bitboard,               // Currently checking pieces
    pub(crate) pinned: Bitboard,                 // Currently pinned pieces
}

/// Some piece/bitboard manipulation functions.
impl Board {

    /// Returns the bitboard of a player.
    #[inline]
    pub fn color(&self, col: Color) -> Bitboard {
        self.colors[col.index()]
    }

    /// Returns the bitboard of a specific piece type.
    #[inline]
    pub fn piece_type(&self, ptype: PieceType) -> Bitboard {
        self.pieces[ptype.index()]
    }

    /// Returns the bitboard of pieces owned by the current player.
    #[inline]
    pub fn own_color(&self) -> Bitboard {
        self.color(self.turn)
    }

    /// Returns the bitboard of pieces owned by the opponent.
    #[inline]
    pub fn opponent_color(&self) -> Bitboard {
        self.color(self.turn.opponent())
    }

    /// Returns the bitboard of a piece owned by the current player.
    #[inline]
    pub fn own_piece_type(&self, ptype: PieceType) -> Bitboard {
        self.color(self.turn) & self.piece_type(ptype)
    }

    /// Returns the bitboard of a piece owned by the opponent.
    #[inline]
    pub fn opponent_piece_type(&self, ptype: PieceType) -> Bitboard {
        self.color(self.turn.opponent()) & self.piece_type(ptype)
    }

    /// Returns the bitboard of a piece of a color.
    #[inline]
    pub fn piece(&self, pc: Piece) -> Bitboard {
        self.color(pc.color) & self.piece_type(pc.ptype)
    }

    /// An utility / alias function for `Board::piece`.
    #[inline]
    pub fn of_color_and_type(&self, col: Color, ptype: PieceType) -> Bitboard {
        self.color(col) & self.piece_type(ptype)
    }

    /// Returns the bitboard of empty squares.
    #[inline]
    pub fn empty(&self) -> Bitboard {
        bit::FULL ^ self.color(White) ^ self.color(Black)
    }

    /// Returns the bitboard of all the pieces.
    #[inline]
    pub fn occupied(&self) -> Bitboard {
        self.color(White) | self.color(Black)
    }

    /// Whether a square is vacant.
    /// 
    /// ```
    /// use chess_std::{Square, Board};
    /// 
    /// let board = Board::default(); // Empty board
    /// for sq in Square::A1..=Square::H8 {
    ///     assert!(board.is_empty(sq));
    /// }
    /// ```
    #[inline]
    pub fn is_empty(&self, sq: Square) -> bool {
        self.empty().get(sq)
    }

    /// Whether a square is occupied by a piece.
    /// 
    /// ```
    /// use chess_std::{Square, Board};
    /// 
    /// let board = Board::new();
    /// assert!(board.is_occupied(Square::E1));
    /// ```
    #[inline]
    pub fn is_occupied(&self, sq: Square) -> bool {
        self.occupied().get(sq)
    }

    /// The color of the piece at a square.
    /// 
    /// ```
    /// use chess_std::{Square, Board};
    /// 
    /// let board = Board::new();
    /// assert_eq!(board.color_at(Square::E4), None);
    /// ```
    #[inline]
    pub fn color_at(&self, sq: Square) -> Option<Color> {
        if self.color(White).get(sq) {
            Some(White)
        } else if self.color(Black).get(sq) {
            Some(Black)
        } else {
            None
        }
    }

    /// The piece type at a square, if any.
    /// 
    /// ```
    /// use chess_std::{Square, PieceType};
    /// use chess_std::Board;
    /// 
    /// let board = Board::new();
    /// assert_eq!(board.piece_type_at(Square::E1), Some(PieceType::King));
    /// ```
    #[inline]
    pub fn piece_type_at(&self, sq: Square) -> Option<PieceType> {    
        if !self.occupied().get(sq) {
            None
        } else if self.piece_type(Pawn).get(sq) {
            Some(Pawn)
        } else if self.piece_type(Knight).get(sq) {
            Some(Knight)
        } else if self.piece_type(Bishop).get(sq) {
            Some(Bishop)
        } else if self.piece_type(Rook).get(sq) {
            Some(Rook)
        } else if self.piece_type(Queen).get(sq) {
            Some(Queen)
        } else {
            Some(King)
        }
    }

    /// The piece at a square, if any.
    /// ```
    /// use chess_std::prelude::*;
    /// use chess_std::Board;
    /// 
    /// let board = Board::new();
    /// for sq in Square::A2..=Square::H2 {
    ///     assert_eq!(board.piece_at(sq), Some(W_PAWN));
    /// }
    /// ```
    #[inline]
    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        Some(Piece { color: self.color_at(sq)?, ptype: self.piece_type_at(sq)? })
    }

    // Add a piece at an empty square.
    #[inline]
    pub(crate) fn add_piece(&mut self, pc: Piece, sq: Square) -> &Self {
        self.pieces[pc.ptype.index()].add(sq);
        self.colors[pc.color.index()].add(sq);
        self.hash ^= zobrist::hash_piece(pc, sq);
        self
    }

    // Remove a piece that was already set.
    #[inline]
    pub(crate) fn remove_piece(&mut self, pc: Piece, sq: Square) -> &Self {
        self.pieces[pc.ptype.index()].remove(sq);
        self.colors[pc.color.index()].remove(sq);
        self.hash ^= zobrist::hash_piece(pc, sq);
        self
    }

    // Move a piece to an empty square.
    pub(crate) fn move_piece(&mut self, pc: Piece, from: Square, to: Square) -> &Self {
        self.remove_piece(pc, from);
        self.add_piece(pc, to)
    }
}

impl Default for Board {
    /// An empty board.
    fn default() -> Self {
        use bit::EMPTY as E;
        let mut empty = Board{
            pieces: [E, E, E, E, E, E],
            colors: [E, E],
            hash: zobrist::INITIAL_HASH,
            turn: White,

            half_move_clock: 0,
            ep_target: None,
            rights: ALL_PLAYERS_RIGHTS,
            last_cap_or_push: 0,

            checkers: bit::EMPTY,
            pinned: bit::EMPTY,
        };
        empty.rehash();
        empty
    }
}


/// Positional functions: getters, attacks, pins.
impl Board {

    /// The initial configuration, without storing move generator.
    pub fn new() -> Board {
        Board{
            pieces: INITIAL_GRID,
            colors: INITIAL_COLORS,
            hash: zobrist::INITIAL_HASH,
            turn: White,

            half_move_clock: 0,
            ep_target: None,
            rights: ALL_PLAYERS_RIGHTS,
            last_cap_or_push: 0,

            checkers: bit::EMPTY,
            pinned: bit::EMPTY,
        }
    }

    /// Returns the number of moves played since the beginning of the game.
    pub fn num_moves_played(&self) -> u32 {
        self.half_move_clock * 2 + match self.turn {
            White => 0,
            Black => 1,
        }
    }

    /// Get the pieces that check the current king.
    pub fn checkers(&self) -> Bitboard {
        self.checkers
    }

    /// Get the pinned pieces.
    pub fn pinned(&self) -> Bitboard {
        self.pinned
    }

    /// Get the square where an en passant capture would be possible.
    pub fn en_passant_target(&self) -> Option<Square> {
        self.ep_target
    }

    // Whether a player has the right to castle in a side.
    #[inline]
    pub fn has_right(&self, player: Color, side: Side) -> bool {
        self.rights[player.index()][side.index()]
    }

    // Add a castling right for a player.
    #[inline]
    pub fn add_right(&mut self, player: Color, side: Side) {
        self.rights[player.index()][side.index()] = true;
    }

    // Remove a castling right for a player.
    #[inline]
    pub fn remove_right(&mut self, player: Color, side: Side) {
        self.rights[player.index()][side.index()] = false;
    }

    // Remove all castling rights for a player.
    #[inline]
    pub fn remove_rights(&mut self, player: Color) {
        self.remove_right(player, Side::King);
        self.remove_right(player, Side::Queen);
    }


    /// Whether a square is directly threatened by pieces of a color
    /// (without necessarily having a legal move at this square).
    pub fn is_attacked(&self, sq: Square, by: Color) -> bool {
        use crate::attack::*;
        let me = by.opponent();
        let ours = self.color(me);
        let enemy = self.color(by);
        let enm = |ptype| enemy & self.piece_type(ptype);
        of_bishop(sq, ours, enemy).intersects(enm(Bishop) | enm(Queen)) ||
        of_rook  (sq, ours, enemy).intersects(enm(Rook)   | enm(Queen)) ||
        of_knight(sq, ours).intersects(enm(Knight)) ||
        of_pawn(me, sq, enemy).intersects(enm(Pawn)) ||
        of_king(sq, ours).intersects(enm(King))
    }

    /// Whether moving a piece to a square may not leave it en prise.
    pub fn is_safe_to_move(&self, from: Square, to: Square) -> bool {
        use crate::attack::*;
        let me = self.color_at(from).unwrap();
        let ours = self.color(me) ^ bit::single(from);
        let enemy = self.color(me.opponent()) & !bit::single(to);
        let enm = |ptype| enemy & self.piece_type(ptype);
        let attackers = (of_bishop(to, ours, enemy) & (enm(Bishop) | enm(Queen)))
                    |   (of_rook  (to, ours, enemy) & (enm(Rook)   | enm(Queen)))
                    |   (of_knight(to, ours)    & enm(Knight))
                    |   (of_pawn(me, to, enemy) & enm(Pawn)  )
                    |   (of_king(to, ours)      & enm(King)  );
        attackers.is_empty()
    }

    /// Whether a square is safe for a color.
    pub fn is_safe(&self, sq: Square, for_: Color) -> bool {
        !self.is_attacked(sq, for_.opponent())
    }


    /// Find the king on the board, assuming the position is legal.
    pub fn king_square_of(&self, player: Color) -> Square {
        self.of_color_and_type(player, King).scan_forward()
    }

    /// The current king.
    pub fn king_square(&self) -> Square {
        self.king_square_of(self.turn)
    }

    /// Whether the current king is checked.
    pub fn is_king_checked(&self) -> bool {
        self.checkers.pop_count() > 0
    }

    /// Whether a piece is pinned to the current king.
    pub fn is_pinned(&self, sq: Square) -> bool {
        self.pinned.get(sq)
    }

    // Update pinners and checkers.
    pub(crate) fn update_attacks(&mut self) {
        use crate::attack::*;
        self.pinned = bit::EMPTY;
        self.checkers = bit::EMPTY;
        let ksq = self.king_square();
        let bishops = self.opponent_piece_type(Bishop);
        let rooks = self.opponent_piece_type(Rook);
        let queens = self.opponent_piece_type(Queen);
        let pinners = (bishop_rays(ksq) & (bishops | queens)) |
                      (  rook_rays(ksq) & (rooks   | queens));
        for pinner in pinners {
            let pinned = fill_between(ksq, pinner) & self.occupied();
            match pinned.pop_count() {
                0 => self.checkers.add(pinner), // No pinned piece to stop the ray
                1 => self.pinned |= pinned,     // A single piece is pinned
                _ => {}
            }
        }
        let pawns = self.opponent_piece_type(Pawn);
        let knights = self.opponent_piece_type(Knight);
        self.checkers |= of_knight(ksq, self.own_color()) & knights;
        self.checkers |= of_pawn(self.turn, ksq, self.opponent_color()) & pawns;
    }

    /// The selected piece of a move.
    #[inline]
    pub fn moved_by(&self, mv: Move) -> Piece {
        self.piece_at(mv.from).unwrap()
    }

    /// The selected piece type of a move.
    #[inline]
    pub fn type_moved_by(&self, mv: Move) -> PieceType {
        self.piece_type_at(mv.from).unwrap()
    }

    /// The eventual captured piece by a move.
    #[inline]
    pub fn captured_by(&self, mv: Move) -> Option<Piece> {
        if let MoveFlag::EnPassant(passed) = mv.flag {
            self.piece_at(passed)
        } else {
            self.piece_at(mv.to)
        }
    }

    /// Whether this position may theoretically occur.
    /// 
    /// ```
    /// use chess_std::Board;
    /// 
    /// let board = Board::new();
    /// assert!(board.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        use crate::attack;
        let is_color_valid = |col| {
            let cnt = |ptype| (self.piece_type(ptype) & self.color(col)).pop_count();
            cnt(Pawn)   <=  8 &&
            cnt(Knight) <= 10 &&
            cnt(Bishop) <= 10 &&
            cnt(Rook)   <= 10 &&
            cnt(Queen)  <=  9 &&
            cnt(King)   ==  1
        };
        if !is_color_valid(Black) || !is_color_valid(White) {
            return false;
        }
        if self.color(Black).intersects(self.color(White)) {
            return false;
        }
        let mut bb = bit::EMPTY;
        for ptype in &ALL_PIECE_TYPES {
            let pc_bb = self.piece_type(*ptype);
            if pc_bb.intersects(bb) {
                return false;
            }
            bb |= pc_bb;
        }
        let opponent = self.turn.opponent();
        let ksq = self.king_square_of(opponent);
        if (self.empty() | bb) != bit::FULL {
            // Color bitboards aren't the entire intersection of piece bitboards.
            return false;
        }
        if !self.is_safe(ksq, opponent) {
            // The opponent king can be captured.
            return false;
        }
        if attack::of_king(self.king_square(), self.own_color()).get(ksq) {
            // Kings are touching.
            return false;
        }
        if let Some(passed_sq) = self.ep_target {
            if !self.opponent_piece_type(Pawn).get(passed_sq) {
                // En passant target is not an opponent pawn.
                return false;
            }
        }
        // Verify consistency of castling rights.
        for col in &PLAYERS {
            for side in &[Side::King, Side::Queen] {
                if self.has_right(*col, *side) {
                    let kfrom = Move::castling_coords(*col, *side, King).0;
                    if !self.of_color_and_type(*col, King).get(kfrom) {
                        // King has moved.
                        return false;
                    }
                    let rfrom = Move::castling_coords(*col, *side, Rook).0;
                    if !self.of_color_and_type(*col, Rook).get(rfrom) {
                        // Rook has moved.
                        return false;
                    }
                }
            }
        }
        true
    }


    /// A unique hash.
    #[inline]
    pub fn zobrist_hash(&self) -> zobrist::Hash {
        self.hash
        ^ zobrist::hash_color(self.turn)
        ^ zobrist::hash_rights(self.rights)
        ^ if let Some(sq) = self.ep_target {
            zobrist::hash_square(sq)
        } else {
            zobrist::NONE_HASH
        }
    }

    pub(crate) fn rehash(&mut self) -> &Self {
        self.hash = zobrist::INITIAL_HASH;
        for pc in &ALL_PIECES {
            let bb_self = self.piece(*pc);
            let bb_initial = INITIAL_GRID[pc.ptype.index()]
                & INITIAL_COLORS[pc.color.index()];
            for sq in bb_self {
                if !bb_initial.get(sq) {
                    self.hash ^= zobrist::hash_piece(*pc, sq); // A piece was added
                }
            }
            for sq in bb_initial {
                if !bb_self.get(sq) {
                    self.hash ^= zobrist::hash_piece(*pc, sq); // A piece was removed
                }
            }
        }
        self
    }

    /// Return a 'pretty' Unicode board representation.
    pub fn to_unicode(&self) -> String {
        let mut s = "  a b c d e f g h".to_string();
        for r in (Rank::R1..=Rank::R8).rev() {
            s.push('\n');
            s.push(r.to_char());
            for f in File::A..=File::H {
                let at = self.piece_at(Square::new(r, f));
                s.push(if let Some(pc) = at { pc.symbol() } else { '-' });
            }
        }
        s
    }

    /// Serialize the piece grid to a 96-byte array.
    pub fn to_bytes(&self) -> [u8; 8 * NUM_PIECE_TYPES] {
        let mut arr = [0u8; 8 * NUM_PIECE_TYPES];
        for (i, bb) in self.pieces.iter().enumerate() {
            for (j, b) in bb.to_bytes().iter().enumerate() {
                arr[i*8 + j] = *b;
            }
        }
        arr
    }
}

/// A fast equality check, using zobrist hashes.
impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        self.hash == other.hash &&
        self.turn == other.turn &&
        self.rights == other.rights && 
        self.ep_target == other.ep_target
    }
}

impl Eq for Board {}

use std::hash::{Hash, Hasher};

// Zobrist hashing.
impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.zobrist_hash().hash(state);
    }
}


#[cfg(test)]
mod board_test {
    use super::*; 

    #[test]
    fn test_at() {
        let board = Board::new();
        for pc in &ALL_PIECES {
            for sq in INITIAL_GRID[pc.ptype.index()] & INITIAL_COLORS[pc.color.index()] {
                assert_eq!(board.color_at(sq), Some(pc.color));
                assert_eq!(board.piece_type_at(sq), Some(pc.ptype));
                assert_eq!(board.piece_at(sq), Some(*pc));
            }
        }
    }

    #[test]
    fn test_zobrist() {
        let mut board = Board::new();
        for pc in &ALL_PIECES {
            for sq in board.piece(*pc) {
                board.remove_piece(*pc, sq);
            }
        }
        assert_eq!(board, Board::default());
        board = Board::new();
        assert_eq!(Board::from_fen(&board.to_fen()).unwrap(), board);
    }
}