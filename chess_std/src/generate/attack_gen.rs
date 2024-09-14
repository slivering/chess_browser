use std::fs;
use std::io::{Write, Result as IoResult};

use crate::units::*;
use crate::bit::*;
use crate::units::{Color::*, Direction::*, PieceType::*};


// Generate rays and pseudo moves.
pub fn write_in(f: &mut fs::File) -> IoResult<()> {
    let rays = build_rays();
    write!(f, "const RAYS: [Grid<Bitboard>; Direction::NUM] = ")?;
    write_bb_grids(f, &rays)?;

    write!(f, "const PSEUDO_MOVES: [Grid<Bitboard>; NUM_PIECE_TYPES] = ")?;
    write_bb_grids(f, &build_pseudo_moves(&rays))?;

    write!(f, "const LINES: Grid<Grid<Bitboard>> = ")?;
    write_bb_grids(f, &build_lines())?;

    writeln!(f, "const DIR_BETWEEN: [[Direction; Square::NUM]; Square::NUM] = [")?;
    for dir_grid in build_dir_between(&rays).iter() {
        write!(f, "    [")?;
        for dir in dir_grid.iter() {
            write!(f, "{:?}, ", dir)?;
        }
        writeln!(f, "],")?;
    }
    writeln!(f, "];")?;

    let (pawn_pushes, pawn_attacks) = build_pawn_moves();
    write!(f, "const PAWN_PUSHES: [Grid<Bitboard>; NUM_PLAYERS] = ")?;
    write_bb_grids(f, &pawn_pushes)?;
    write!(f, "const PAWN_ATTACKS: [Grid<Bitboard>; NUM_PLAYERS] = ")?;
    write_bb_grids(f, &pawn_attacks)?;
    Ok(())
}


fn write_bb_grids(f: &mut fs::File, bb_grids: &[Grid<Bitboard>]) -> IoResult<()> {
    writeln!(f, "[")?;
    for bb_grid in bb_grids {
        writeln!(f, "    [")?;
        for bb in bb_grid.iter() {
            writeln!(f, "        {:?},", bb)?;
        }
        writeln!(f, "],")?;
    }
    writeln!(f, "];")?;
    Ok(())
}

fn build_rays() -> [Grid<Bitboard>; Direction::NUM] {   
    let mut rays = [[EMPTY; Square::NUM]; Direction::NUM];

    let mut bb = FILE_A ^ single(Square::A1);
    let mut set_ray = |dir: Direction, sq: Square, v: Bitboard| {
        rays[dir.index()][sq.index()] = v;
    };

    for sq in Square::A1..=Square::H8 {
        set_ray(North, sq, bb);
        bb.0 <<= 1;
    }
    bb = FILE_H ^ single(Square::H8);
    for sq in (Square::A1..=Square::H8).rev() {
        set_ray(South, sq, bb);
        bb.0 >>= 1;
    }
    bb = RANK_1;
    for r in Rank::R1..=Rank::R8 {
        let mut ray = bb;
        for f in File::A..=File::H {
            ray = ray.shift(East);
            set_ray(East, Square::new(r, f), ray);
        }
        ray = bb;
        for f in (File::A..=File::H).rev() {
            ray = ray.shift(West);
            set_ray(West, Square::new(r, f), ray);
        }
        bb = bb.shift(North);
    }
    bb = DIAG_A1_H8 ^ single(Square::A1);
    for r in Rank::R1..=Rank::R8 {
        let mut ray = bb;
        for f in File::A..=File::H {
            set_ray(NorthEast, Square::new(r, f), ray);
            ray = ray.shift(East);
        }
        bb = bb.shift(North);
    }
    bb = DIAG_A8_H1 ^ single(Square::H1);
    for r in Rank::R1..=Rank::R8 {
        let mut ray = bb;
        for f in (File::A..=File::H).rev() {
            set_ray(NorthWest, Square::new(r, f), ray);
            ray = ray.shift(West);
        }
        bb = bb.shift(North);
    }
    bb = DIAG_A8_H1 ^ single(Square::A8);
    for r in (Rank::R1..=Rank::R8).rev() {
        let mut ray = bb;
        for f in File::A..=File::H {
            set_ray(SouthEast, Square::new(r, f), ray);
            ray = ray.shift(East);
        }
        bb = bb.shift(South);
    }
    bb = DIAG_A1_H8 ^ single(Square::H8);
    for r in (Rank::R1..=Rank::R8).rev() {
        let mut ray = bb;
        for f in (File::A..=File::H).rev() {
            set_ray(SouthWest, Square::new(r, f), ray);
            ray = ray.shift(West);
        }
        bb = bb.shift(South);
    }
    rays
}


fn build_pseudo_moves(rays: &[Grid<Bitboard>; Direction::NUM])
                          -> [Grid<Bitboard>; NUM_PIECE_TYPES] {
    let get_ray = |dir: Direction, from: Square|
    rays[dir.index()][from.index()];
    let mut pseudo_moves = [[EMPTY; Square::NUM]; NUM_PIECE_TYPES];
    
    for sq in Square::A1..=Square::H8 {
        let bb = single(sq);
        let mut set_moves = |ptype: PieceType, v: Bitboard|
            pseudo_moves[ptype.index()][sq.index()] = v;
        
        set_moves(Pawn, bb.shift(NorthWest) | bb.shift(NorthEast));
        set_moves(Knight,
            bb.shift(North).shift(NorthEast) |
            bb.shift(North).shift(NorthWest) |
            bb.shift(South).shift(SouthWest) |
            bb.shift(South).shift(SouthEast) |
            bb.shift(East).shift(NorthEast)  |
            bb.shift(East).shift(SouthEast)  |
            bb.shift(West).shift(NorthWest)  |
            bb.shift(West).shift(SouthWest));
        set_moves(Bishop,
            get_ray(North, sq) |
            get_ray(South, sq) |
            get_ray(East, sq)  |
            get_ray(West, sq));
        set_moves(Rook,
            get_ray(NorthWest, sq) |
            get_ray(NorthEast, sq) |
            get_ray(SouthWest, sq) |
            get_ray(SouthEast, sq));
        set_moves(Queen,
            get_ray(North, sq) |
            get_ray(South, sq) |
            get_ray(East, sq)  |
            get_ray(West, sq)  |
            get_ray(NorthWest, sq) |
            get_ray(NorthEast, sq) |
            get_ray(SouthWest, sq) |
            get_ray(SouthEast, sq)
        );
        set_moves(King,
            bb.shift(North)  |
            bb.shift(South)  |
            bb.shift(East)   |
            bb.shift(West)   |
            bb.shift(NorthWest) |
            bb.shift(NorthEast) |
            bb.shift(SouthWest) |
            bb.shift(SouthEast));
    }
    pseudo_moves
}

type PlayerGrid = [Grid<Bitboard>; NUM_PLAYERS];

// Returns the pawn pushes and attacks for each player.
fn build_pawn_moves() -> (PlayerGrid, PlayerGrid) {
    let mut pushes = [[EMPTY; Square::NUM]; NUM_PLAYERS];
    let mut attacks = pushes;
    for sq in Square::A1..=Square::H8 {
        let bb = single(sq);
        attacks[White.index()][sq.index()] = bb.shift(NorthWest) | bb.shift(NorthEast);
        attacks[Black.index()][sq.index()] = bb.shift(SouthWest) | bb.shift(SouthEast);
        for col in &PLAYERS {
            let dir = Direction::of_pawns(*col);
            let mut push = bb.shift(dir);
            if sq.rank() == Rank::of_pawns(*col) {
                push |= push.shift(dir);
            }
            pushes[col.index()][sq.index()] = push;
        }
    }
    (pushes, attacks)
}


fn build_dir_between(rays: &[Grid<Bitboard>; Direction::NUM])
                     -> Grid<Grid<Direction>> {
    let get_ray = |dir: Direction, from: Square|
        rays[dir.index()][from.index()];
    let mut dir_btw = [[NoDir; Square::NUM]; Square::NUM];
    for from in Square::A1..=Square::H8 {
        for dir in &ALL_DIRECTIONS {
            for to in get_ray(*dir, from) {
                dir_btw[from.index()][to.index()] = *dir;
            }
        }
    }
    dir_btw
}


fn build_lines() -> Grid<Grid<Bitboard>> {
    let mut lines = [[EMPTY; Square::NUM]; Square::NUM];
    let mut set_lines = |mut bb: Bitboard, dir: Direction, n: usize| {
        for _ in 0..n {
            for sq1 in bb {
                for sq2 in bb {
                    lines[sq1.index()][sq2.index()] = bb;
                }
            }
            bb = bb.shift(dir);
        }
    };
    set_lines(RANK_1, North, 8);
    set_lines(FILE_A, East, 8);
    set_lines(DIAG_A1_H8, NorthWest, 4);
    set_lines(DIAG_A1_H8, SouthEast, 4);
    set_lines(DIAG_A8_H1, NorthEast, 4);
    set_lines(DIAG_A8_H1, SouthWest, 4);
    
    lines
}