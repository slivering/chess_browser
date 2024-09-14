#![feature(test)]

extern crate test;

use test::{Bencher};
use chess_std::*;

#[bench]
fn bench_board_creation(b: &mut Bencher) {
    let def = Board::new();
    b.iter(|| {
        let board = Board::new();
        assert_eq!(board, def);
    });
}

#[bench]
fn bench_game(b: &mut Bencher) {
    let mut num_iterations = 0;
    let mut avg_num_moves = 0;
    b.iter(|| {
        let mut i = 0;
        let mut game = Game::new();
        avg_num_moves += game.moves.len();
        num_iterations += 1;
        while !game.is_finished() && !game.can_claim_draw() {
            i += 1;
            let mv = game.legal_moves()
                .nth(i % game.board().num_moves())
                .unwrap();
            game.play_move(mv);
        }
    });
}

#[bench]
fn bench_board_until_over(b: &mut Bencher) {
    let mut num_iterations = 0;
    let mut avg_num_moves = 0;
    b.iter(|| {
        let mut i = 0;
        let mut board = Board::new();
        while !board.is_finished() && !board.can_claim_draw() {
            i += 1;
            avg_num_moves += board.num_moves();
            num_iterations += 1;
            let mv = board.legal_moves()
                .nth(i % board.num_moves())
                .unwrap();
            board = board.play_move(mv);
        }
    });
}