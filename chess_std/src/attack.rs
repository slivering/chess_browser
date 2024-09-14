use crate::prelude::*;
use crate::bit::*;
use crate::units::Direction::{self, *};


include!("./generate/attack_tables.rs");


#[inline]
fn get_ray(dir: Direction, from: Square) -> Bitboard {
    unsafe {
        *RAYS
            .get_unchecked(dir.index())
            .get_unchecked(from.index())
    }
}

/// The direction in which a piece must move from a square to reach another,
/// assuming both squares are different.
/// 
/// ```
/// use chess_std::{Square, Direction, attack};
/// 
/// assert_eq!(attack::direction_between(Square::A1, Square::H8), Direction::NorthEast);
/// ```
#[inline]
pub fn direction_between(from: Square, to: Square) -> Direction {
    unsafe {
        *DIR_BETWEEN
            .get_unchecked(from.index())
            .get_unchecked(to.index())
    }
}

/// The squares between an origin and a destination.
/// 
/// ```
/// use chess_std::{Square, bit::{self, single}, attack::fill_between};
/// 
/// assert_eq!(fill_between(Square::H1, Square::A8),
///            bit::DIAG_A8_H1 ^ single(Square::A8) ^ single(Square::H1));
/// ```
#[inline]
pub fn fill_between(from: Square, to: Square) -> Bitboard {
    let dir = direction_between(from, to);
    get_ray(dir, from) ^ get_ray(dir, to) ^ single(to)
}

/// The line that contains two squares and extends until edges.
/// 
/// ```
/// use chess_std::{Square, bit, attack};
/// 
/// assert_eq!(attack::fill_line(Square::C6, Square::F6), bit::RANK_6);
/// ```
#[inline]
pub fn fill_line(from: Square, to: Square) -> Bitboard {
    unsafe {
        *LINES
            .get_unchecked(from.index())
            .get_unchecked(to.index())
    }
}

/// "fill" a ray attack towards a direction. The ray will be blocked by
/// the first blocker if any, but also include the blocker square
/// if it is an enemy.
#[inline(always)]
pub fn fill(dir: Direction, from: Square, same_color: Bitboard,
            enemy: Bitboard) -> Bitboard {
    let mut ray = get_ray(dir, from);
    let blockers = (same_color | enemy) & ray;
    if blockers.is_populated() {
        let blocker = if dir as i8 > 0 {
            blockers.scan_forward()
        } else {
            blockers.scan_reverse()
        };
        ray ^= get_ray(dir, blocker);
    }
    ray & !same_color // Not to capture capture friend pieces!
}

/// The pawn pushes and double pushes.
/// 
/// ```
/// # #[macro_use]
/// # extern crate chess_std;
/// use chess_std::{Color, Square};
/// use chess_std::{bit, attack};
/// 
/// # fn main() {
/// assert_eq!(attack::pawn_pushes(Color::White, Square::E2, bit::EMPTY),
///            merge_sq!(Square::E3, Square::E4));
/// 
/// let blockers = bit::single(Square::B6);
/// assert_eq!(attack::pawn_pushes(Color::Black, Square::B7, blockers),
///            bit::EMPTY);
/// # }
/// ```
#[inline]
pub fn pawn_pushes(col: Color, from: Square, blockers: Bitboard) -> Bitboard {
    let dir = Direction::of_pawns(col);
    // Very important. Disallows to double push
    // when a blocker is just in front of the pawn.
    let no_reach = (single(from).shift(dir) & blockers).shift(dir);
    unsafe {
        *PAWN_PUSHES
            .get_unchecked(col.index())
            .get_unchecked(from.index())
        & !no_reach
        & !blockers
    }
}

/// The pawn diagonal attacks.
/// 
/// ```
/// # #[macro_use]
/// # extern crate chess_std;
/// use chess_std::{Color, Square};
/// use chess_std::{bit, attack};
/// 
/// # fn main() {
/// let enemy = bit::single(Square::B4);
/// let attacks = attack::of_pawn(Color::Black, Square::C5, enemy);
/// assert_eq!(attacks, bit::single(Square::B4));
/// # }
/// ```
#[inline]
pub fn of_pawn(col: Color, from: Square, enemy: Bitboard) -> Bitboard {
    unsafe {
        *PAWN_ATTACKS
            .get_unchecked(col.index())
            .get_unchecked(from.index())
        & enemy
    }
}

/// The knight attacks.
/// ```
/// use chess_std::{Square, bit, attack};
/// 
/// let same_color = bit::RANK_2 | bit::single(Square::C3);
/// let attacks = attack::of_knight(Square::B1, same_color);
/// assert_eq!(attacks, bit::single(Square::A3));
/// ```
#[inline]
pub fn of_knight(from: Square, same_color: Bitboard) -> Bitboard {
    unsafe {
        *PSEUDO_MOVES
            .get_unchecked(Knight.index())
            .get_unchecked(from.index())
        & !same_color
    }
}

/// The diagonal rays from a square.
#[inline]
pub fn bishop_rays(from: Square) -> Bitboard {
    use Direction::*;
    get_ray(NorthWest, from) |
    get_ray(NorthEast, from) |
    get_ray(SouthWest, from) |
    get_ray(SouthEast, from)
}

/// The bishop attacks.
/// 
/// ```
/// # #[macro_use]
/// # extern crate chess_std;
/// use chess_std::{Square, Direction};
/// use chess_std::{bit::{self, single}, attack};
/// 
/// # fn main() {
///  let (same_color, enemy) = (single(Square::B2), single(Square::H8));
///  let diagonals = (bit::DIAG_A1_H8 | bit::DIAG_A8_H1.shift(Direction::West));
///  let attacks = attack::of_bishop(Square::D4, same_color, enemy);
///  assert_eq!(attacks, diagonals ^ merge_sq!(Square::A1, Square::B2, Square::D4));
/// # }
/// ```
#[inline]
pub fn of_bishop(from: Square, same_color: Bitboard, enemy: Bitboard) -> Bitboard {
    fill(NorthWest, from, same_color, enemy) |
    fill(NorthEast, from, same_color, enemy) |
    fill(SouthWest, from, same_color, enemy) |
    fill(SouthEast, from, same_color, enemy)
}

/// The horizontal and vertical rays from a square.
#[inline]
pub fn rook_rays(from: Square) -> Bitboard {
    use Direction::*;
    get_ray(North, from) |
    get_ray(South, from) |
    get_ray(West, from)  |
    get_ray(East, from)
}

/// The rook attacks.
/// 
/// ```
/// # #[macro_use]
/// # extern crate chess_std;
/// use chess_std::Square;
/// use chess_std::{bit::{self, single}, attack};
/// 
/// # fn main() {
/// let same_color = single(Square::H5);
/// let enemy = single(Square::A5);
/// let attacks = attack::of_rook(Square::E5, same_color, enemy);
/// let cross = bit::FILE_E | bit::RANK_5;
/// let expected = cross ^ merge_sq!(Square::E5, Square::H5);
/// assert_eq!(attacks, expected);
/// # }
/// ```
#[inline]
pub fn of_rook(from: Square, same_color: Bitboard, enemy: Bitboard) -> Bitboard {
    fill(North, from, same_color, enemy) |
    fill(South, from, same_color, enemy) |
    fill(West,  from, same_color, enemy) |
    fill(East,  from, same_color, enemy)
}

/// The queen attacks.
#[inline]
pub fn of_queen(from: Square, same_color: Bitboard, enemy: Bitboard) -> Bitboard {
    of_bishop(from, same_color, enemy) | of_rook(from, same_color, enemy)
}

/// The king attacks.
/// 
/// ```
/// # #[macro_use]
/// # extern crate chess_std;
/// use chess_std::{Square, bit, attack};
/// 
/// # fn main() {
/// let same_color = bit::single(Square::G7);
/// let attacks = attack::of_king(Square::H8, same_color);
/// assert_eq!(attacks, merge_sq!(Square::G8, Square::H7));
/// # }
/// ```
#[inline]
pub fn of_king(from: Square, same_color: Bitboard) -> Bitboard {
    unsafe {
        *PSEUDO_MOVES
            .get_unchecked(King.index())
            .get_unchecked(from.index())
        & !same_color
    }   
}