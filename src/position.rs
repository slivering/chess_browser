use wasm_bindgen::prelude::*;

use chess_std as cs;
use crate::units::{Square, Color, PieceType, Piece};
use crate::moves::Move;


/// A `Board` is a representation of the game that views, modifies the position.
/// It can store precalculated legal moves and can apply them on a successor.
/// 
/// Use this instead of `Game` for performance, if knowing the previous boards
/// and moves is not needed.
#[wasm_bindgen]
#[derive(Clone, PartialEq, Eq)]
pub struct Board(pub (crate) cs::Board);

#[wasm_bindgen]
impl Board {

    pub fn copy(&self) -> Self {
        Self(self.0.clone())
    }

    pub fn equals(&self, other: &Board) -> bool {
        self.0 == other.0
    }

    /// The initial configuration.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(cs::Board::new())
    }

    /// An empty board.
    pub fn empty() -> Self {
        Self(cs::Board::default())
    }

    /// Builds a Board from the FEN notation.
    #[wasm_bindgen(catch)]
    pub fn fromFen(fen: &str) -> Result<Board, JsValue> {
        cs::Board::from_fen(fen).map(Self)
            .map_err(|err| js_sys::Error::new(&err).into())
    }

    /// Returns the positional FEN notation of this `Board`.
    pub fn toFen(&self) -> String {
        self.0.to_fen()
    }

    #[wasm_bindgen(getter)]
    pub fn turn(&self) -> Color {
        Color(self.0.turn)
    }

    /// The number of similar pieces on the board.
    pub fn countPiece(&self, pc: &Piece) -> u32 {
        self.0.piece(pc.0).pop_count()
    }

    /// The number of pieces of a specific type on the board.
    pub fn countPieceType(&self, ptype: &PieceType) -> u32 {
        self.0.piece_type(ptype.0).pop_count()
    }

    /// The number of pieces of a player.
    pub fn countColor(&self, col: &Color) -> u32 {
        self.0.color(col.0).pop_count()
    }

    /// The number of all pieces on the board.
    pub fn countAllPieces(&self) -> u32 {
        self.0.occupied().pop_count()
    }

    /// The number of empty squares on the board.
    pub fn countEmpty(&self) -> u32 {
        self.0.empty().pop_count()
    }

    /// Whether a square is vacant.
    pub fn isEmpty(&self, sq: &Square) -> bool {
        self.0.is_empty(sq.cs())
    }

    /// Whether a square is occupied by a piece.
    pub fn isOccupied(&self, sq: &Square) -> bool {
        !self.0.is_occupied(sq.cs())
    }

    /// The color of the piece at a square. Returns `undefined` when none.
    pub fn colorAt(&self, sq: &Square) -> Option<Color> {
        self.0.color_at(sq.cs()).map(Color)
    }

    /// The type of a piece at a square. Returns `undefined` when none.
    pub fn pieceTypeAt(&self, sq: &Square) -> Option<PieceType> {
        self.0.piece_type_at(sq.cs()).map(PieceType)
    }

    /// The piece at a square. Returns `undefined` when none.
    pub fn pieceAt(&self, sq: &Square) -> Option<Piece> {
        self.0.piece_at(sq.cs()).map(Piece)
    }
    
    /// If a square is directly threatened by pieces of a color
    /// (without necessarily having a legal move at this square).
    pub fn isAttacked(&self, sq: &Square, by: &Color) -> bool {
        self.0.is_attacked(sq.cs(), by.0)
    }

    /// Find the king on the board, assuming the position is legal.
    pub fn kingSquareOf(&self, player: &Color) -> Square {
        Square::from_cs(self.0.king_square_of(player.0))
    }

    pub fn isKingChecked(&self) -> bool {
        self.0.is_king_checked()
    }

    /// The selected piece of a move.
    pub fn movedBy(&self, mv: &Move) -> Piece {
        Piece(self.0.moved_by(mv.cs()))
    }

    /// The eventual captured piece by a move. Returns `undefined` when none.
    pub fn capturedBy(&self, mv: &Move) -> Option<Piece> {
        self.0.captured_by(mv.cs()).map(Piece)
    }

    /// Apply the move in place.
    pub fn applyMove(&mut self, mv: &Move) {
        self.0.apply_move(mv.cs());
    }

    /// Whether this position may theoretically occur.
    pub fn isValid(&self) -> bool {
        self.0.is_valid()
    }

    /// A unique hash.
    pub fn zobristHash(&self) -> u64 {
        self.0.zobrist_hash()
    }

    /// Return a 'pretty' Unicode board representation.
    pub fn toUnicodeStr(&self) -> String {
        self.0.to_unicode()
    }

    pub fn toString(&self) -> String {
        format!("{:?}", self.0)
    }
}