// Copyright 2020 lvt
//
// Licensed under the MIT license.

//! Chess interface and file parsing
//! 
//! This crate provides a fast game representation that supports
//! the official FIDE rules, a game tree structure, file parsing and formatting.
//! It notably supports the PGN file format with SAN notation and
//! FEN notation for position encoding.
//! 
//! ## A basic example
//! ```
//! use chess_std::Game;
//! 
//! let mut game = Game::new();
//! 
//! while !game.is_finished() && !game.can_claim_draw() {
//!     // A simple ASCII string
//!     println!("{}\n\n", game.board());
//!     let mv = game.legal_moves().next().unwrap();
//!     assert!(game.is_move_legal(mv), "Illegal move: {}", mv);
//!     game.play_move(mv);
//! }
//! println!("Final FEN:\n{}\nPGN:\n`{}`", game.board().to_fen(), game.to_pgn());
//! if game.is_finished() {
//!     // The game is either checkmate or stalemate
//!     println!("Game over by {}", game.result);
//! } else {
//!     // A draw is detected
//!     println!("Game drawn by {:?}", game.get_draw_type());
//! }
//! ```
//! 
//! ## More about move generation
//! ```
//! use chess_std::Game;
//! 
//! let mut game = Game::new();
//! 
//! println!("Before:\n{}\n", game.board());
//! 
//! // Create a move generator
//! let mut gen = game.legal_moves();
//! 
//! // The exact size of the generator
//! let n = gen.len();
//! println!("Number of moves: {}", n);
//! 
//! // Iterate over the legal moves
//! for i in 0..(n-1) {
//!     println!("- {}", game.board().pgn_move(gen.next().unwrap()));
//! }
//! 
//! // Finally, play the last move
//! let mv = gen.next().unwrap();
//! println!("\nAfter move {}:", game.board().pgn_move(mv));
//! game.play_move(mv);
//! println!("{:?}", game.board());
//! ```

#![crate_type = "lib"]
#![crate_name = "chess_std"]
#![feature(step_trait)] // FIXME: change this


#[macro_use]
mod macros;

#[macro_use]
mod units;
pub use units::Direction;

pub mod prelude;
pub use prelude::*;

#[macro_use]
pub mod bit;
pub use bit::Bitboard;

pub mod attack;

mod moves;
pub use moves::{CheckType, castling};

mod position;
pub use position::Board;

mod state; // Import the implementation

mod builder;

pub mod board {
    pub use crate::position::{zobrist, Board};
    pub use crate::builder::Builder;
}

mod movegen;
pub use movegen::{MoveGenMasked, MoveGen, MoveGenerator};

mod game;
pub use game::{Game, GameResult, WinType, DrawType};

#[cfg(feature = "pgn")]
pub use {moves::PGNMove, game::PGNTags};

#[cfg(feature = "trees")]
pub use game::{Tree, TreeNode, TreeIterator};