/* The implementation of `Bitboard` and piece attacks. */

use derive_more::{Add, Sub, Mul, BitAnd, BitOr, BitXor, Not,
                  BitAndAssign, BitOrAssign, BitXorAssign, Binary, From, Into};
use std::fmt;

use crate::units::{Rank, File, Square};
use crate::units::Direction::{self, *};

/// A `Bitboard` is a binary set of squares of length 64.
#[derive(Add, Sub, Mul, Not, BitAnd, BitOr, BitXor,
         BitAndAssign, BitOrAssign, BitXorAssign, Binary, From, Into)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct Bitboard(pub u64);

newtype_values! {
    pub const Bitboard {
        EMPTY = 0,
        FULL  = 0xffff_ffff_ffff_ffff,
        DARK_SQUARES = 0xAA55AA55AA55AA55,
        LIGHT_SQUARES = !DARK_SQUARES.0,
        RANK_1 = 0xff,
        RANK_2 = RANK_1.0 << 8,
        RANK_3 = RANK_2.0 << 8,
        RANK_4 = RANK_3.0 << 8,
        RANK_5 = RANK_4.0 << 8,
        RANK_6 = RANK_5.0 << 8,
        RANK_7 = RANK_6.0 << 8,
        RANK_8 = RANK_7.0 << 8,
        FILE_A = 0x101_0101_0101_0101,
        FILE_B = FILE_A.0 << 1,
        FILE_C = FILE_B.0 << 1,
        FILE_D = FILE_C.0 << 1,
        FILE_E = FILE_D.0 << 1,
        FILE_F = FILE_E.0 << 1,
        FILE_G = FILE_F.0 << 1,
        FILE_H = FILE_G.0 << 1,
        DIAG_A1_H8 = 0x8040_2010_0804_0201,
        DIAG_A8_H1 = 0x102_0408_1020_4080,
        W_RANKS = RANK_1.0 | RANK_2.0,
        B_RANKS = RANK_7.0 | RANK_8.0
    };
}




/// Returns a `Bitboard` that contains a single `Square`.
/// 
/// ```
/// use chess_std::{bit, Square, Bitboard};
/// assert_eq!(bit::single(Square::H1), Bitboard(128));
/// ```
#[inline(always)]
pub const fn single(sq: Square) -> Bitboard {
    Bitboard(1u64 << (sq.0 as u64))
}


impl Bitboard {
    /// The index of ls1b. Returns `Square(64)` on empty sets.
    /// ```
    /// use chess_std::{Square, bit, Bitboard};
    /// assert_eq!(Bitboard(0b0000_1000).scan_forward(), Square::D1);
    /// assert_eq!(bit::FULL.scan_forward(), Square::A1);
    /// assert_eq!(bit::EMPTY.scan_forward(), Square::from(64));
    /// ```
    #[inline(always)]
    pub fn scan_forward(self) -> Square {
        Square(self.0.trailing_zeros() as u8)
    }

    /// The index of ms1b. Returns `Square(64)` on empty sets.
    /// ```
    /// use chess_std::{Square, bit, Bitboard};
    /// assert_eq!(Bitboard(0b1000_0000).scan_reverse(), Square::H1);
    /// assert_eq!(bit::FULL.scan_reverse(), Square::H8);
    /// assert_eq!(bit::EMPTY.scan_reverse(), Square::from(64));
    /// ```
    #[inline(always)]
    pub fn scan_reverse(self) -> Square {
        if self.is_populated() {
            Square(self.0.leading_zeros() as u8 ^ 63u8)
        } else {
            Square(64)
        }
    }

    /// Returns whether the set is populated (non-zero).
    #[inline(always)]
    pub fn is_populated(self) -> bool {
        self.0 != 0
    }

    /// Returns whether the set is empty (zero).
    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns whether a `Square` belongs to the set.
    /// 
    /// ```
    /// use chess_std::{Square, bit};
    /// 
    /// assert!(bit::RANK_1.get(Square::D1));
    /// ```
    #[inline(always)]
    pub fn get(self, sq: Square) -> bool {
        (self.0 & (1u64 << sq.0 as u64)) != 0
    }

    /// Add a `Square` to the set.
    /// 
    /// ```
    /// use chess_std::{Square, bit};
    /// 
    /// let mut bb = bit::RANK_1;
    /// for sq in bit::RANK_2 {
    ///     bb.add(sq);
    /// }
    /// assert_eq!(bb, bit::RANK_1 | bit::RANK_2);
    /// ```
    #[inline(always)]
    pub fn add(&mut self, sq: Square) {
        *self |= single(sq);
    }

    /// Remove a `Square` from the set, present or not.
    /// 
    /// ```
    /// use chess_std::{Square, bit};
    /// 
    /// let mut bb = bit::RANK_1 | bit::RANK_2;
    /// for sq in bit::RANK_2 {
    ///     bb.remove(sq);
    /// }
    /// assert_eq!(bb, bit::RANK_1);
    /// ```
    #[inline(always)]
    pub fn remove(&mut self, sq: Square) {
        *self &= !single(sq);
    }

    /// Returns whether both sets have squares in common.
    /// 
    /// ```
    /// use chess_std::bit;
    /// 
    /// assert!(bit::DIAG_A1_H8.intersects(bit::RANK_5));
    /// ```
    pub fn intersects(self, bb: Self) -> bool {
        self.0 & bb.0 != 0
    }

    /// Returns a new set with all the squares of the set shifted towards a direction.
    /// 
    /// ```
    /// use chess_std::{Square, Direction::*, bit::{self, EMPTY, single}};
    /// 
    /// assert_eq!(bit::RANK_1.shift(South), EMPTY);
    /// assert_eq!(bit::RANK_8.shift(North), EMPTY);
    /// assert_eq!(bit::FILE_A.shift(West),  EMPTY);
    /// assert_eq!(bit::FILE_H.shift(East),  EMPTY);
    /// 
    /// let bb = single(Square::D4);
    /// assert_eq!(bb.shift(NorthWest).shift(SouthEast), bb);
    /// assert_eq!(bb.shift(SouthWest).shift(NorthEast), bb);
    /// ```
    #[inline(always)]
    pub fn shift(self, dir: Direction) -> Self {
        let mut bb = self.0;
        bb = match dir {
            North       =>  bb              <<  8,
            South       =>  bb              >>  8,
            East        => (bb & !FILE_H.0) <<  1,
            NorthEast   => (bb & !FILE_H.0) <<  9,
            SouthEast   => (bb & !FILE_H.0) >>  7,
            West        => (bb & !FILE_A.0) >>  1,
            NorthWest   => (bb & !FILE_A.0) <<  7,
            SouthWest   => (bb & !FILE_A.0) >>  9,
            NoDir       => bb
        };
        Self(bb)
    }

    /// Returns the number of squares in the set (equivalent to the number of ones
    /// in the binary representation).
    #[inline(always)]
    pub fn pop_count(self) -> u32 {
        self.0.count_ones()
    }

    /// Returns a new `Bitboard` with the player views reversed.
    /// This is equivalent to bytes swapping.
    /// 
    /// ```
    /// use chess_std::{Square, bit::{self, single}};
    /// 
    /// assert_eq!((bit::RANK_1 ^ single(Square::A1)).flip_vertical(),
    ///     (bit::RANK_8 ^ single(Square::A8)));    
    /// ```
    #[inline(always)]
    pub fn flip_vertical(self) -> Self {
        Self(self.0.swap_bytes())
    }

    /// Returns a new `Bitboard` with the files mirrored to a horizontal axis.
    /// ```
    /// use chess_std::{Square, bit::{self, single}};
    /// 
    /// let bb = bit::FILE_A ^ single(Square::A8);
    /// assert_eq!(bb.mirror_horizontal(), bit::FILE_H ^ single(Square::H8));
    /// ```
    pub fn mirror_horizontal(self) -> Self {
        let mut bb = self.0;
        let k1: u64 = 0x5555555555555555;
        let k2: u64 = 0x3333333333333333;
        let k4: u64 = 0x0f0f0f0f0f0f0f0f;
        bb = ((bb >> 1) & k1) +  2*(bb & k1);
        bb = ((bb >> 2) & k2) +  4*(bb & k2);
        bb = ((bb >> 4) & k4) + 16*(bb & k4);
        Self(bb)
    }

    /// Returns a new `Bitboard` flipped vertically and mirrored horizontally.
    /// ```
    /// use chess_std::{Square, bit::{self, single}};
    /// let bb = bit::DIAG_A1_H8;
    /// assert_eq!((bb ^ single(Square::A1)).rotate180(), bb ^ single(Square::H8));
    /// ```
    pub fn rotate180(self) -> Self {
        self.flip_vertical().mirror_horizontal()
    }

    #[doc(hidden)]
    pub fn to_bytes(self) -> [u8; 8] {
        unsafe { std::mem::transmute::<Bitboard, [u8; 8]>(self) }
    }
}

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }
        let sq = self.scan_forward();
        self.remove(sq);
        Some(sq)
    }
}



impl fmt::Display for Bitboard {
    fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fm, "  a b c d e f g h")?;
        for r in (Rank::R1..=Rank::R8).rev() {
            write!(fm, "\n{}", r)?;
            for f in File::A..=File::H {
                write!(fm, " {}", if self.get(Square::new(r, f)) {"@"} else {"."})?;
            }
        }
        Ok(())
    }
}

/// Create a new `Bitboard` that contains some squares.
#[macro_export]
macro_rules! merge_sq {
    ($($sq: expr),*) => {
        {
            use crate::bit;
            bit::EMPTY $( | bit::single($sq) )*
        }
    };
}



#[test]
fn test_iter() {
    use crate::units::{Rank, File};
    let mut it = DIAG_A8_H1;
    let mut f = File(8);
    for r in Rank::R1..=Rank::R8 {
        f.0 -= 1;
        assert_eq!(it.next(), Some(Square::new(r, f)));
    }
    assert_eq!(it.next(), None);
}