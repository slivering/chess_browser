use wasm_bindgen::prelude::*;
use chess_std as cs;
use crate::Board;

fn explore(board: cs::Board, depth: u32) -> u32 {
    let mut n = 0;
    if depth == 1 {
        return board.num_moves() as u32;
    }
    for mv in board.legal_moves() {
        n += explore(board.play_move(mv), depth - 1);
    }
    n
}

/// A simple perft test that returns the number of legal moves generated
/// from `board`, after `depth` (depth 1 is the minimum).
#[wasm_bindgen]
pub fn perft(board: &Board, depth: u32) -> u32 {
    explore(board.0.clone(), depth)
}