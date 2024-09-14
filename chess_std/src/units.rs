/* The basic types of chess:
- `Color`
- `PieceType`
- `Piece`
- `File`
- `Rank`
- `Square`
- and utility types: `Direction`
*/

use derive_more::{Add, Sub, From, Into};

use std::fmt;
use std::convert::TryFrom;

use Color::*;
use PieceType::*;
pub use Direction::*;


/// The designation of a player.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Color {
    White  = 0,
    Black  = 1
}

/// There are two players.
pub const NUM_PLAYERS: usize = 2;

/// Both players.
pub const PLAYERS: [Color; NUM_PLAYERS] = [Color::White, Color::Black];

impl Color {

    /// The opponent of the player.
    #[inline]
    pub fn opponent(self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }

    #[inline]
    pub(crate) fn index(self) -> usize {
        self as usize
    }
}

char_enum_conversions! {
    match Color {
        White => 'w',
        Black => 'b'
    }
}



/// The role of a piece, which determines its moves.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum PieceType {
    Pawn   = 0,
    Knight = 1,
    Bishop = 2,
    Rook   = 3,
    Queen  = 4,
    King   = 5,
}

/// The number of piece types.
pub const NUM_PIECE_TYPES: usize = 6;

/// All the piece types.
pub const ALL_PIECE_TYPES: [PieceType; NUM_PIECE_TYPES] = [
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
];

impl PieceType {
    
    /// The relative piece value.
    pub fn value(self) -> u8 {
        match self {
            Pawn   => 1,
            Knight => 3,
            Bishop => 3,
            Rook   => 5,
            Queen  => 9,
            King   => 255,
        }
    }

    /// If a pawn can promote into this piece type.
    #[inline]
    pub fn can_be_promotion(self) -> bool {
        self > Pawn && self < King
    }

    #[inline]
    pub(crate) fn index(self) -> usize {
        self as usize
    }
}

char_enum_conversions! {
    match PieceType {
        Pawn   => 'P',
        Knight => 'N',
        Bishop => 'B',
        Rook   => 'R',
        Queen  => 'Q',
        King   => 'K'
    }
}



/// A `Piece` is owned by a player and has a type.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub ptype: PieceType,
}

pub const W_PAWN:   Piece = Piece{color: White, ptype: Pawn};
pub const W_KNIGHT: Piece = Piece{color: White, ptype: Knight};
pub const W_BISHOP: Piece = Piece{color: White, ptype: Bishop};
pub const W_ROOK:   Piece = Piece{color: White, ptype: Rook};
pub const W_QUEEN:  Piece = Piece{color: White, ptype: Queen};
pub const W_KING:   Piece = Piece{color: White, ptype: King};
pub const B_PAWN:   Piece = Piece{color: Black, ptype: Pawn};
pub const B_KNIGHT: Piece = Piece{color: Black, ptype: Knight};
pub const B_BISHOP: Piece = Piece{color: Black, ptype: Bishop};
pub const B_ROOK:   Piece = Piece{color: Black, ptype: Rook};
pub const B_QUEEN:  Piece = Piece{color: Black, ptype: Queen};
pub const B_KING:   Piece = Piece{color: Black, ptype: King};

/// The number of pieces, considering their color.
pub const NUM_PIECES: usize = 12;


/// The white pieces.
pub const WHITE_PIECES: [Piece; NUM_PIECE_TYPES] = [
    W_PAWN,
    W_KNIGHT,
    W_BISHOP,
    W_ROOK,
    W_QUEEN,
    W_KING,
];

/// The black pieces.
pub const BLACK_PIECES: [Piece; NUM_PIECE_TYPES] = [
    B_PAWN,
    B_KNIGHT,
    B_BISHOP,
    B_ROOK,
    B_QUEEN,
    B_KING,
];

/// All the pieces.
pub const ALL_PIECES: [Piece; NUM_PIECES] = [
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


impl Piece {

    pub(crate) fn index(&self) -> usize {
        6 * self.color.index() + self.ptype.index()
    }

    /// The SAN notation of a piece.
    /// 
    /// ```
    /// use chess_std::prelude::*;
    /// 
    /// for ptype in &ALL_PIECE_TYPES {
    ///     let white = Piece{color: Color::White, ptype: *ptype}.to_char();
    ///     let black = Piece{color: Color::Black, ptype: *ptype}.to_char();
    ///     assert_eq!(white, black.to_ascii_uppercase());
    /// }
    /// ```
    pub fn to_char(self) -> char {
        let mut c = self.ptype.to_char();
        if self.color == Black {
            c = c.to_ascii_lowercase();
        }
        c
    }

    /// The unicode symbol of the `Piece`.
    #[inline]
    pub fn symbol(&self) -> char {
        const SYMBOLS: [char; NUM_PIECES] = [
            '\u{2659}',
            '\u{2658}',
            '\u{2657}',
            '\u{2656}',
            '\u{2655}',
            '\u{2654}',
            '\u{265f}',
            '\u{265e}',
            '\u{265d}',
            '\u{265c}',
            '\u{265b}',
            '\u{265a}',
        ];
        SYMBOLS[self.index()]
    }
}

impl TryFrom<char> for Piece {
    type Error = String;
    fn try_from(mut c: char) -> Result<Self, Self::Error> {
        let color = if c.is_uppercase() { White } else { Black };
        c = c.to_ascii_uppercase();
        Ok(Piece{ ptype: c.try_into()?, color })
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}



/// A rank is a row of the board, from 1 to 8.
/// The first rank's value is `R1 = 0`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[derive(Add, Sub, From, Into, Hash)]
pub struct Rank(pub(crate) u8);

impl_step!(Rank);

impl Rank {

    newtype_values! {
        pub const Rank {
            R1 = 0, R2 = 1, R3 = 2, R4 = 3, R5 = 4, R6 = 5, R7 = 6, R8 = 7
        };
    }
    
    pub const NUM: usize = 8;

    /// Assuming this rank is in White's perspective, this returns
    /// a vertically flipped rank for Black, and is a no-op for White.
    /// 
    /// ```
    /// use chess_std::{Color, Rank};
    /// 
    /// assert_eq!(Rank::R3.relative(Color::White), Rank::R3);
    /// assert_eq!(Rank::R3.relative(Color::Black), Rank::R6);
    /// ```
    #[inline]
    pub fn relative(self, col: Color) -> Self {
        match col {
            Color::White => self,
            Color::Black => Self::R8 - self
        }
    }
    
    /// The relative rank 1.
    #[inline]
    pub fn first(col: Color) -> Self {
        Self::relative(Self::R1, col)
    }

    /// The relative rank 2.
    #[inline]
    pub fn of_pawns(col: Color) -> Self {
        Self::relative(Self::R2, col)
    }

    /// The relative rank 8.
    #[inline]
    pub fn last(col: Color) -> Self {
        Self::relative(Self::R8, col)
    }

    /// Returns the corresponding digit of this `Rank`.
    /// 
    /// ```
    /// use chess_std::Rank;
    /// 
    /// assert_eq!(Rank::R1.to_char(), '1');
    /// assert_eq!(Rank::R8.to_char(), '8');
    /// ```
    pub fn to_char(self) -> char {
        (b'1' + self.0) as char
    }

    /// Parse the rank from a digit.
    /// 
    /// ```
    /// use chess_std::Rank;
    /// 
    /// assert_eq!(Rank::from_char('1'), Ok(Rank::R1));
    /// assert_eq!(Rank::from_char('8'), Ok(Rank::R8));
    /// ```
    pub fn from_char(c: char) -> Result<Self, String> {            
        if ('1'..='8').contains(&c) {
            Ok(Self(c as u8 - b'1'))
        } else {
            Err(format!("Invalid rank: `{}`", c))
        }
    }
}

impl fmt::Debug for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}


/// A file is a column of the board, from `A` to `H`.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[derive(Add, Sub, From, Into, Hash)]
pub struct File(pub(crate) u8);

impl_step!(File);

impl File {

    newtype_values! {
        pub const File {
            A = 0, B = 1, C = 2, D = 3, E = 4, F = 5, G = 6, H = 7
        };
    }
    
    pub const NUM: usize = 8;

    /// Convert the file to its corresponding lowercase letter.
    #[inline]
    pub fn to_char(self) -> char {
        (b'a' + self.0) as char
    }

    /// Parse the file from a lowercase char (from `'a'` to `'h'`).
    pub fn from_char(c: char) -> Result<Self, String> {
        if ('a'..='h').contains(&c) {
            Ok(Self(c as u8 - b'a'))
        } else {
            Err(format!("Invalid file: `{}`", c))
        }
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}



/// There are 64 Squares on the board.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[derive(Add, Sub, From, Into, Hash)]
pub struct Square(pub(crate) u8);

impl_step!(Square);

impl Square {

    newtype_values! {
        pub const Square {
            A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
            A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
            A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
            A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
            A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
            A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
            A2 =  8, B2 =  9, C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
            A1 =  0, B1 =  1, C1 =  2, D1 =  3, E1 =  4, F1 =  5, G1 =  6, H1 =  7,
            NONE = 64
        };
    }

    /// The number of squares on the board.
    pub const NUM: usize = 64;

    /// A square from a file and a rank.
    /// ```
    /// use chess_std::{Rank, File, Square};
    /// assert_eq!(Square::new(Rank::R1, File::G), Square::G1);
    /// for r in Rank::R1..=Rank::R8 {
    ///     for f in File::A..=File::H {
    ///         let sq = Square::new(r, f);
    ///         assert_eq!(sq.rank(), r);
    ///         assert_eq!(sq.file(), f);
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn new(r: Rank, f: File) -> Self {
        Self((r.0 << 3) + f.0)
    }

    /// The rank of a square.
    #[inline]
    pub fn rank(self) -> Rank {
        Rank(self.0 >> 3)
    }

    /// The file of a square.
    #[inline]
    pub fn file(self) -> File {
        File(self.0 & 7)
    }

    /// Returns the number of ranks that separate two squares.
    /// ```
    /// use chess_std::Square;
    /// assert_eq!(Square::D4.rank_distance(Square::G2), 2);
    /// ```
    #[inline]
    pub fn rank_distance(self, other: Self) -> usize {
        (self.rank().0 as i32 - other.rank().0 as i32).abs() as usize
    }

    /// Returns the number of files that separate two squares.
    /// ```
    /// use chess_std::Square;
    /// assert_eq!(Square::G4.file_distance(Square::D2), 3);
    /// ```
    #[inline]
    pub fn file_distance(self, other: Self) -> usize {
        (self.file().0 as i32 - other.file().0 as i32).abs() as usize
    }

    /// Assuming this square is in White's perspective, this returns
    /// a vertically flipped square for Black, and is a no-op for White.
    /// ```
    /// use chess_std::{Color, Square};
    /// assert_eq!(Square::B2.relative(Color::Black), Square::B7);
    /// ```
    #[inline]
    pub fn relative(self, player: Color) -> Self {
        Self::new(self.rank().relative(player), self.file())
    }

    /// A square such as Square::NONE may not be on the board.
    #[inline]
    pub fn is_on_board(self) -> bool {
        self.0 < Square::NUM as u8
    }

    /// Returns whether the color of the square on the chessboard is dark (brown).
    /// 
    /// ```
    /// use chess_std::Square;
    /// 
    /// assert!(Square::A1.is_dark() && Square::H8.is_dark());
    /// ```
    #[inline]
    pub fn is_dark(self) -> bool {
        ((self.rank().0 & 1) + (self.file().0 & 1)) % 2 == 0
    }

    /// Returns the lowercase SAN notation of a square.
    /// 
    /// ```
    /// use chess_std::Square;
    /// 
    /// assert_eq!(Square::A6.san(), "a6");
    /// ```
    pub fn san(self) -> String {
        let r = self.file().to_char();
        let f = self.rank().to_char();
        [r, f].iter().clone().collect()
    }

    /// Creates a square from SAN notation, either lowercase or uppercase.
    /// 
    /// ```
    /// use chess_std::Square;
    /// 
    /// assert_eq!(Square::from_san("H8"), Ok(Square::H8));
    /// ```
    pub fn from_san(san: &str) -> Result<Square, String> {
        let san = san.to_ascii_lowercase();
        if san.len() != 2 {
            Err(format!("Couldn't parse square SAN: `{}`", san))
        } else {
            let get_char = |n: usize| san.as_bytes()[n] as char;
            let f = File::from_char(get_char(0))?;
            let r = Rank::from_char(get_char(1))?;
            Ok(Self::new(r, f))
        }
    }

    /// Shift this `Square` in a direction. This operation might fail
    /// when the square is near to the edge.
    /// 
    /// ```
    /// use chess_std::{Square, Direction};
    /// 
    /// println!("{}", Square::H1.shift(Direction::East)); // Prints `a2`
    /// ```
    #[inline]
    pub fn shift(self, dir: Direction) -> Square {
        let shifted = self.0 as i8 + dir as i8;
        if shifted > 0 { Square(shifted as u8) } else { Square::NONE }
    }

    /// This swaps the view of the players.
    /// 
    /// ```
    /// use chess_std::Square;
    /// 
    /// assert_eq!(Square::F3.flip_vertical(), Square::F6);
    /// ```
    #[inline]
    pub fn flip_vertical(self) -> Self {
        Self(self.0 ^ 56)
    }

    /// A symmetry on the mid-horizontal axis.
    #[inline]
    pub fn mirror_horizontal(self) -> Self {
        Self(self.0 ^ 7)
    }

    /// This is equivalent to `self.flip_vertical().mirror_horizontal()`.
    #[inline]
    pub fn rotate180(self) -> Self {
        Self(self.0 ^ 63)
    }

    /// An utility function to use a `Square` as an index.
    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.san())
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.san())
    }
}

pub type Grid<T> = [T; Square::NUM];


/// The 9 basic directions, useful to compute piece moves.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[repr(i8)]
pub enum Direction {
    NorthWest =  7, North =  8, NorthEast =  9,
         West = -1, NoDir =  0,      East =  1,
    SouthWest = -9, South = -8, SouthEast = -7,
}

#[allow(dead_code)]
pub const ALL_DIRECTIONS: [Direction; 8] = [
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
];

impl Direction {
    
    pub const NUM: usize = 8;

    /// Get the direction in which pawns of a color move.
    #[inline]
    pub fn of_pawns(col: Color) -> Self {
        match col {
            White => North,
            Black => South,
        }
    }

    /// An utility function that allows to index a `Direction`.
    #[inline]
    pub fn index(self) -> usize {
        match self {
            North  => 0,
            South  => 1,
            East   => 2,
            West   => 3,
            NorthWest  => 4,
            NorthEast  => 5,
            SouthWest  => 6,
            SouthEast  => 7,
            NoDir      => 8
        }
    }
}


impl std::ops::Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            North  => South,
            South  => North,
            East   => West,
            West   => East,
            NorthWest  => SouthEast,
            NorthEast  => SouthWest,
            SouthWest  => NorthEast,
            SouthEast  => NorthWest,
            NoDir      => NoDir
        }
    }
}



#[test]
fn test_char_conversions() {
    assert_eq!(White.to_char(), 'w');
    assert_eq!(Pawn.to_char(),  'P');
    assert_eq!(King.to_char(),  'K');

    assert_eq!(Color::try_from('b'),     Ok(Black));
    assert!(Color::try_from('x').is_err());
    assert_eq!(PieceType::try_from('B'), Ok(Bishop));
    assert!(PieceType::try_from('-').is_err());
    assert_eq!(Piece::try_from('P'),     Ok(W_PAWN));
    assert_eq!(Piece::try_from('r'),     Ok(B_ROOK));
}
