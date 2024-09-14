
#[cfg(test)]
mod perft {
    use std::time::{Instant};

    use chess_std::*;

    fn explore(board: Board, depth: u32) -> u32 {
        let mut n = 0;
        if depth == 1 {
            return board.num_moves() as u32;
        }
        for mv in board.legal_moves() {
            n += explore(board.play_move(mv), depth - 1);
        }
        n
    }

    fn timed_explore(name: &str, board: Board, depth: u32, expected: u32) {
        let t0 = Instant::now();
        let n = explore(board, depth);
        let t1 = Instant::now();
        let millis = (t1 - t0).as_nanos() as f64 / 1e6;
        let millions = n as f64 / 1e6;
        let rate = millions * 1e3 / millis as f64;
        println!("\nTest results for {}:", name);
        let num_fmt = if n < 1_000 {
            format!("{}", n)
        } else if n < 1_000_000 {
            format!("{:.2}K", n as f64 / 1e3)
        } else {
            format!("{:.2}M", n as f64 / 1e6)
        };
        println!("  Generated {} moves in {:.2} ms ({:.2}M/s)", num_fmt, millis, rate);
        if n != expected {
            let e  = n as i32 - expected as i32;
            println!("  error {}", e);
        }
    }

    #[test]
    fn all() {
        movegen_begin();
        movegen_kiwipete();
        movegen_1_2();
        movegen_3_4();
        movegen_5_6();
        movegen_7_8();
        movegen_9_10();
        movegen_11_12();
        movegen_13_14();
        movegen_15_16();
        movegen_17_18();
        movegen_19_20();
        movegen_21_22();
        movegen_23_24();
        movegen_25_26();
    }
    
    fn movegen_begin() {
        timed_explore("Start-1", Board::new(), 1, 20);
        timed_explore("Start-2", Board::new(), 2, 400);
        timed_explore("Start-3", Board::new(), 3, 8902);
        timed_explore("Start-4", Board::new(), 4, 197281);
        // -56
        timed_explore("Start-5", Board::new(), 5, 4865609);
        // -7232
        timed_explore("Start-6", Board::new(), 6, 119060324);
    }

    // Might overflow over 4 billion moves
    fn test_expect(name: &str, fen: &str, depth: u32, expected: u32) {
        let board = Board::from_fen(fen).unwrap();
        timed_explore(name, board, depth, expected)
    }

    fn movegen_kiwipete() {
        // -14571
        test_expect(
            "kiwipete",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            5,
            193690690,
        );
    }

    fn movegen_1_2() {
        // -36
        test_expect("1", "8/5bk1/8/2Pp4/8/1K6/8/8 w - d6 0 1", 6, 824064);
        test_expect("2", "8/8/1k6/8/2pP4/8/5BK1/8 b - d3 0 1", 6, 824064);
    }

    fn movegen_3_4() {
        // -35103
        test_expect("3", "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1", 6, 1440467);
        test_expect("4", "8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1", 6, 1440467);
    }

    fn movegen_5_6() {
        test_expect("5", "5k2/8/8/8/8/8/8/4K2R w K - 0 1", 6, 661072);
        test_expect("6", "4k2r/8/8/8/8/8/8/5K2 b k - 0 1", 6, 661072);

    }

    fn movegen_7_8() {
        test_expect("7", "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1", 6, 803711);
        test_expect("8", "r3k3/8/8/8/8/8/8/3K4 b q - 0 1", 6, 803711);
    }

    fn movegen_9_10() {
        // -1
        test_expect(
            "9",
            "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1",
            4,
            1274206,
        );
        test_expect(
            "10",
            "r3k2r/7b/8/8/8/8/1B4BQ/R3K2R b KQkq - 0 1",
            4,
            1274206,
        );
    }

    fn movegen_11_12() {
        // -82
        test_expect(
            "11",
            "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1",
            4,
            1720476,
        );
        test_expect(
            "12",
            "r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1",
            4,
            1720476,
        );
    }

    fn movegen_13_14() {
        test_expect("13", "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1", 6, 3821001);
        test_expect("14", "3K4/8/8/8/8/8/4p3/2k2R2 b - - 0 1", 6, 3821001);
    }

    fn movegen_15_16() {
        test_expect("15", "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1", 5, 1004658);
        test_expect("16", "5K2/8/1Q6/2N5/8/1p2k3/8/8 w - - 0 1", 5, 1004658);

    }

    fn movegen_17_18() {
        test_expect("17", "4k3/1P6/8/8/8/8/K7/8 w - - 0 1", 6, 217342);
        test_expect("18", "8/k7/8/8/8/8/1p6/4K3 b - - 0 1", 6, 217342);
    }

    fn movegen_19_20() {
        test_expect("19", "8/P1k5/K7/8/8/8/8/8 w - - 0 1", 6, 92683);
        test_expect("20", "8/8/8/8/8/k7/p1K5/8 b - - 0 1", 6, 92683);
    }

    fn movegen_21_22() {
        test_expect("21", "K1k5/8/P7/8/8/8/8/8 w - - 0 1", 6, 2217);
        test_expect("22", "8/8/8/8/8/p7/8/k1K5 b - - 0 1", 6, 2217);

    }

    fn movegen_23_24() {
        test_expect("23", "8/k1P5/8/1K6/8/8/8/8 w - - 0 1", 7, 567584);
        test_expect("24", "8/8/8/8/1k6/8/K1p5/8 b - - 0 1", 7, 567584);

    }

    fn movegen_25_26() {
        test_expect("25", "8/5k2/8/5N2/5Q2/2K5/8/8 w - - 0 1", 4, 23527);
        test_expect("26", "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1", 4, 23527);
    }

}
