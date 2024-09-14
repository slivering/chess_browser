// My shameless minimax.

use wasm_bindgen::prelude::*;

use chess_std as cs;
use super::Engine;
use crate as wasm;

type Score = i32;

const MIN_SCORE: Score = i32::MAX;
const MAX_SCORE: Score = i32::MIN;
const AVG_SCORE: Score = 0;


/// A basic, exhaustive minimax engine.
#[wasm_bindgen]
pub struct Minimax {
    depth: u32
}

impl Default for Minimax {
    fn default() -> Self {
        Self { depth: 4 }
    }
}

impl Engine for Minimax {
    fn select_move(&mut self, board: cs::Board) -> Option<cs::Move> {
        self.move_with_best_score(board, AVG_SCORE, self.depth).0
    }
}

#[wasm_bindgen]
impl Minimax {
    /// Create a new engine from a search depth.
    /// 
    /// It must be an even, non-zero value.
    #[wasm_bindgen(constructor, catch)]
    pub fn new(depth: u32) -> Result<Minimax, JsValue> {
        if depth == 0 {
            Err("Cannot have depth-0 minimax engine".into())
        } else if depth % 2 != 0 {
            Err("Search depth must be even".into())
        } else {
            Ok(Self { depth })
        }
    }

    /// Get the search depth of this engine.
    #[wasm_bindgen(getter)]
    pub fn depth(&self) -> u32 {
        self.depth
    }

    /// Select a move from a board. Returns `undefined` when no move can be selected.
    #[wasm_bindgen]
    pub fn selectMove(&mut self, board: wasm::Board) -> Option<wasm::Move> {
        self.select_move(board.0).map(wasm::Move::from_cs)
    }

    // Find the best move to play if any, and the resulting score after playing it.
    fn move_with_best_score(&self, board: cs::Board,
                            current_score: Score, depth: u32)
                            -> (Option<cs::Move>, Score) {
        match board.get_result() {
            cs::GameResult::Win(winner, _) => {
                return if winner == board.turn {
                    (None, MAX_SCORE)
                } else {
                    (None, MIN_SCORE)
                };
            },
            cs::GameResult::Draw(_) => return (None, AVG_SCORE),
            _ => {}
        };
        if depth < self.depth {
            // Return the current positional evaluation.
            return (None, current_score);
        }
        let mut best_move: Option<cs::Move> = None;
        let mut best_score = current_score;
        for mv in board.legal_moves() {
            let mut next_score = current_score;
            if let Some(piece) = board.captured_by(mv) {
                // Update the positional score,
                // based on the piece captured by the current player.
                next_score += piece.ptype.value() as Score;
            };
            let next_board = board.play_move(mv);
            let (_, best_opponent_score) = self.move_with_best_score(
                next_board, -next_score, depth - 1);
            // We want the opposite of our opponent.
            let our_score = -best_opponent_score;
            if our_score > best_score {
                best_move = Some(mv);
                best_score = our_score;
            }
        }
        (best_move, best_score)
    }
}