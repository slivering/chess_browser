import * as wasm from "../pkg/chess_browser";


export const WHITE: wasm.Color = wasm.Color.fromChar('w');
export const BLACK: wasm.Color = wasm.Color.fromChar('b');

export const PLAYERS: wasm.Color[] = [WHITE, BLACK];

export const PAWN: wasm.PieceType   = wasm.PieceType.fromChar('P');
export const KNIGHT: wasm.PieceType = wasm.PieceType.fromChar('N');
export const BISHOP: wasm.PieceType = wasm.PieceType.fromChar('B');
export const ROOK: wasm.PieceType   = wasm.PieceType.fromChar('R');
export const QUEEN: wasm.PieceType  = wasm.PieceType.fromChar('Q');
export const KING: wasm.PieceType   = wasm.PieceType.fromChar('K');

/** The number of pieces. */
export const NUM_PIECE_TYPES: number = 6;

/** All the piece types. */
export const ALL_PIECE_TYPES: wasm.PieceType[] = [
     PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING
 ];

export const W_PAWN: wasm.Piece    = wasm.Piece.fromChar('P');
export const W_KNIGHT: wasm.Piece  = wasm.Piece.fromChar('N');
export const W_BISHOP: wasm.Piece  = wasm.Piece.fromChar('B');
export const W_ROOK: wasm.Piece    = wasm.Piece.fromChar('R');
export const W_QUEEN: wasm.Piece   = wasm.Piece.fromChar('Q');
export const W_KING: wasm.Piece    = wasm.Piece.fromChar('K');

export const B_PAWN: wasm.Piece    = wasm.Piece.fromChar('p');
export const B_KNIGHT: wasm.Piece  = wasm.Piece.fromChar('n');
export const B_BISHOP: wasm.Piece  = wasm.Piece.fromChar('b');
export const B_ROOK: wasm.Piece    = wasm.Piece.fromChar('r');
export const B_QUEEN: wasm.Piece   = wasm.Piece.fromChar('q');
export const B_KING: wasm.Piece    = wasm.Piece.fromChar('k');

/** The number of pieces. */
export const NUM_PIECES: number = 12;

/** The white pieces. */
export const WHITE_PIECES: wasm.Piece[] = [
    W_PAWN,
    W_KNIGHT,
    W_BISHOP,
    W_ROOK,
    W_QUEEN,
    W_KING,
];

/** The black pieces. */
export const BLACK_PIECES: wasm.Piece[] = [
    B_PAWN,
    B_KNIGHT,
    B_BISHOP,
    B_ROOK,
    B_QUEEN,
    B_KING,
];

/** All the pieces. */
export const ALL_PIECES: wasm.Piece[] = [
    W_PAWN,
    W_KNIGHT,
    W_BISHOP,
    W_ROOK,
    W_QUEEN,
    W_KING,
    B_PAWN,
    B_KNIGHT,
    B_BISHOP,
    B_ROOK,
    B_QUEEN,
    B_KING,
];