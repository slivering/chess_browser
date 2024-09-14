use wasm_bindgen::prelude::*;
use derive_more::Index;

use chess_std as cs;

use crate::units::{Color, Square};
use crate::moves::{self, Move};
use crate::position::Board;
use crate::state::{GameResult, DrawType};


/// A wrapper around Board that stores the boards and the moves.
/// It can be assimilated to a stack where the last element
/// is the current board.
#[wasm_bindgen]
pub struct Game(cs::Game);




#[wasm_bindgen]
impl Game {
    /// A game that starts with the first board.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(cs::Game::new())
    }

    /// A game that starts from a specific board, as if it were the first.
    pub fn fromBoard(board: &Board) -> Self {
        Self(cs::Game::from_board(board.0.clone()))
    }

    /// The current board.
    #[wasm_bindgen(getter)]
    pub fn board(&self) -> Board {
        Board(self.0.board().clone())
    }

    /// The side to move.
    #[wasm_bindgen(getter)]
    pub fn turn(&self) -> Color {
        Color(self.0.board().turn)
    }

    #[wasm_bindgen(getter)]
    pub fn moves(&self) -> js_sys::Array {
        moves::slice_into_array(&self.0.moves)
    }

    #[wasm_bindgen(getter)]
    pub fn lastMove(&self) -> Option<Move> {
        self.0.moves.last().copied().map(Move::from_cs)
    }

    /// Whether the piece's color at a square is the turn.
    pub fn canSelectSquare(&self, sq: &Square) -> bool {
        self.0.board().color_at(sq.cs()) == Some(self.0.board().turn)
    }

    /// Returns a move from a square and to another,
    /// and `undefined` if nothing was found.
    pub fn moveFromTo(&self, from: &Square, to: &Square) -> Option<Move> {
        let from = from.cs();
        let to = to.cs();
        for mv in self.0.legal_moves_from(from) {
            if mv.from == from && mv.to == to {
                return Some(Move::from_cs(mv));
            }
        }
        None        
    }

    /// See: `Board.legal_moves_from`.
    pub fn legalMovesFrom(&mut self, sq: &Square) -> js_sys::Array {
        moves::gen_into_array(self.0.legal_moves_from(sq.cs()))
    }

    /// All the legal moves.
    pub fn legalMoves(&self) -> js_sys::Array {
        moves::gen_into_array(self.0.legal_moves())
    }

    /// See: `Board.is_move_legal`.
    pub fn isMoveLegal(&mut self, mv: &Move) -> bool {
        self.0.is_move_legal(mv.cs())
    }

    /// Use this function instead of `Game.board().play_move`
    /// to update the game after a move.
    pub fn playMove(&mut self, mv: &Move) {
        self.0.play_move(mv.cs());
    }

    /// Remove the last board and the last move from the list.
    /// The board of the game will then be the previous one.
    pub fn undoLastMove(&mut self) {
        self.0.undo_last_move();
    }

    /// See: `Board.in_checkmate`.
    pub fn inCheckmate(&self) -> bool {
        self.0.in_checkmate()
    }

    /// See: `Board.in_stalemate`.
    pub fn inStalemate(&self) -> bool {
        self.0.in_stalemate()
    }

    /// See: `Board.is_finished`.
    pub fn isFinished(&self) -> bool {
        self.0.is_finished()
    }

    pub fn getResult(&self) -> GameResult {
        GameResult::from_cs(self.0.get_result())
    }

    /// This completes `Board.can_claim_draw_with` for threefold repetition.
    pub fn canClaimDrawWith(&self, dt: DrawType) -> bool {
        self.0.can_claim_draw_with(dt.cs())
    }

    /// This completes `Board.can_claim_draw` for threefold repetition.
    pub fn canClaimDraw(&self) -> bool {
        self.0.can_claim_draw()
    }

    /// Returns a valid draw claim if any, otherwise `undefined`.
    pub fn getDrawType(&self) -> Option<DrawType> {
        self.0.get_draw_type().map(DrawType::from_cs)
    }

    /// Parse PGN game data. tags will be ignored.
    #[wasm_bindgen(catch)]
    pub fn fromPgn(pgn: &str) -> Result<Game, JsValue> {
        cs::Game::from_pgn(pgn).map(Self)
            .map_err(|err| js_sys::Error::new(&err).into())
    }

    /// Parse a PGN move, playable at this state.
    #[wasm_bindgen(catch)]
    pub fn parseMove(&self, pgn: &str) -> Result<Move, JsValue> {
        self.0.parse_move(pgn).map(Move::from_cs)
            .map_err(|_| js_sys::Error::new("Couldn't parse move").into())
    }

    /// Convert this game to a PGN string, without more metadata.
    /// The moves are translated to the long algebraic notation.
    pub fn toPgn(&self) -> String {
        self.0.to_pgn()
    }

}



/// PGN metadata, that consists in tag-pairs.
/// 
/// The tag name is an ASCII string, that indexes the tag value which is
/// a single-line textual string.
#[wasm_bindgen]
#[derive(Debug, Clone, Index)]
pub struct PGNTags(cs::PGNTags);

#[wasm_bindgen]
impl PGNTags {
    pub fn equals(&self, rhs: &PGNTags) -> bool {
        self.0 == rhs.0
    }

    /// New PGNTags without any tag pairs stored.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(cs::PGNTags::new())
    }

    /// Extract tags from PGN.
    pub fn fromPgn(pgn: &str) -> PGNTags {
        Self(cs::PGNTags::from_pgn(pgn))
    }

    /// Add a new ASCII tag with a value as string.
    pub fn addTag(&mut self, tag: &str, value: String) {
        self.0.add_tag(tag, value);
    }

    /// Convert tags to PGN-embeddable string.
    pub fn toString(&self) -> String {
        self.0.to_pgn()
    }
}
