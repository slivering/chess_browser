//! Add `chess_std` common names into scope.
//! 
//! Basic usage:
//! 
//! ```
//! use chess_std::prelude::*;
//! ```

pub use crate::units::{
    Color, Color::{White, Black}, NUM_PLAYERS, PLAYERS,
    PieceType, NUM_PIECE_TYPES, ALL_PIECE_TYPES,
    PieceType::{Pawn, Knight, Bishop, Rook, Queen, King},
    Piece, NUM_PIECES, BLACK_PIECES, WHITE_PIECES, ALL_PIECES,
    W_PAWN, W_KNIGHT, W_BISHOP, W_ROOK, W_QUEEN, W_KING,
    B_PAWN, B_KNIGHT, B_BISHOP, B_ROOK, B_QUEEN, B_KING, 
    Rank, File, Square, Grid
};

pub use crate::moves::{Move, MoveFlag::{self, *}, Moves, castling::Side};