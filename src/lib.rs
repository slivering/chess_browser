// Copyright 2020 lvt
//
// Licensed under the MIT license.

//! WASM Chess interface
//! 
//! This crate provides a browser interface between the engine and the user,
//! using the `chess_std` library with WASM bindings.
//! 
//! Most types here are duplicating the original functions from the wrapped object.

#![crate_type = "dylib"]
#![allow(non_snake_case)]
#![allow(clippy::new_without_default)]

mod engine;

mod units;
pub use units::{Color, PieceType};

mod moves;
pub use moves::{Move, PGNMove, CastlingSide};

mod position;
pub use position::Board;

mod state;
pub use state::{GameResult, WinType, DrawType};

mod game;
pub use game::{Game, PGNTags};

mod perft;
pub use perft::perft;