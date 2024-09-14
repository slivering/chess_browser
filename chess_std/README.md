# chess_std

Chess interface and file parsing.

This crate provides a fast but complete chess implementation. It supports official FIDE rules. It interfaces well with PGN and FEN file formats for game and position encoding.

## Types

The most basic types that form a chessboard are defined:
- `Color`
- `PieceType`
- `Piece`
- `Square`
- `Rank`
- `File`

## Rules

... to be used on a `Board` structure, that enables to play a game from the beginning until the end. It provides fast move generation and making - internally with bitboards. It tracks the result and supports draw claim by threefold-repetition, fifty-move rule and insufficient material.

## Features

- `fen`: The `Board` can be parsed from and converted into a FEN string.
- `pgn`: In addition to store the moves and the positions, the `Game` can be constructed and converted using the PGN file format.
- `trees`: The `GameTree` also enables to build sequences of boards and variations.

## A basic example

```rust
use chess_std::Game;

let mut game = Game::new();

println!("Before:\n{}\n", game.board());

// Create a move generator
let mut gen = game.legal_moves();

// The exact size of the generator
let n = gen.len();
println!("Number of moves: {}", n);

// Iterate over the legal moves
for i in 0..(n-1) {
    println!("- {}", game.board().pgn_move(gen.next().unwrap()));
}

// Finally, play the last move
let mv = gen.next().unwrap();
println!("\nAfter move {}:", game.board().pgn_move(mv));
game.play_move(mv);
println!("{:?}", game.board());
```

## More about move generation

```rust
use chess_std::Game;

let mut game = Game::new();

println!("Before:\n{}\n", game.board());

// Create a move generator
let gen = game.legal_moves();

// The exact size of the generator
println!("Number of moves: {}", gen.len());

// Iterate over the legal moves
for mv in gen.take(gen.len() - 1) {
    println!("- {}", game.board().pgn_move(mv));
}

// Finally, play the last move
let mv = gen.next().unwrap();
println!("\nAfter move {}:", game.pgn_move(mv));
game.play_move(mv);
println!("{:?}", game.board());
```

## What you can do from here

Feel free to use this library in any of your chess projects !