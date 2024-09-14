//! This module provides chess engines made on top of the `chess_std` module.
//! 

use chess_std as cs;

pub mod minimax;


/// A chess engine searches a move.
pub trait Engine {

    // An engine is required to select a move, given a board.
    // It returns `None` when the game is already over.
    fn select_move(&mut self, board: cs::Board) -> Option<cs::Move>;
}