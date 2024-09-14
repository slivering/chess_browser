/* The definitions of a move.
- `Move`: the standard object used by `Board`
- `PGNMove`: provides additional information for the PGN file format.
*/

use crate::prelude::*;
pub use MoveFlag::*;


/// This module provides properties for castling moves.
pub mod castling {
    use super::*;
    use std::convert::TryFrom;

    /// The side of a castling.
    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub enum Side {
        King  = 0,
        Queen = 1
    }

    pub const NUM_SIDES: usize = 2;

    impl From<Side> for PieceType {
        fn from(side: Side) -> PieceType {
            match side {
                Side::King  => King,
                Side::Queen => Queen
            }
        }
    }

    impl TryFrom<PieceType> for Side {
        type Error = String;

        fn try_from(ptype: PieceType) -> Result<Self, Self::Error> {
            match ptype {
                King  => Ok(Self::King),
                Queen => Ok(Self::Queen),
                _                => Err(format!("Cannot convert: {}", ptype))
            }
        }
    }

    impl Side {
        pub(crate) fn index(self) -> usize {
            self as usize
        }
    }

    // The castling rights for a player.
    pub(crate) type Rights = [bool; NUM_SIDES];
    
    pub(crate) const ALL_RIGHTS: Rights = [true, true];
    pub(crate) const NO_RIGHTS:  Rights = [false, false];
}


/// A special move property. Move flags may not be combined.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MoveFlag {
    Quiet,
    EnPassant(Square),
    Promotion(PieceType),
    Castling(castling::Side),
}



/// A minimal move information.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub flag: MoveFlag
}

/// A vector of moves.
pub type Moves = Vec<Move>;


impl Move {
    /// A null move. Does nothing apart from giving the turn.
    pub const NONE: Move = Move{from: Square::NONE, to: Square::NONE, flag: Quiet};

    // squares by player, by side, for the king/rook moves.
    const CASTLINGS: [[[(Square, Square); 2]; castling::NUM_SIDES]; NUM_PLAYERS] = [
        [
            [(Square::E1, Square::G1), (Square::H1, Square::F1)], // White kingside
            [(Square::E1, Square::C1), (Square::A1, Square::D1)], // White queenside
        ],
        [
            [(Square::E8, Square::G8), (Square::H8, Square::F8)], // Black kingside
            [(Square::E8, Square::C8), (Square::A8, Square::D8)], // Black queenside
        ]
    ];

    // Ranks of en passant destinations for each player.
    #[doc(hidden)]
    pub const EN_PASSANT_RANKS: [Rank; 2] = [Rank::R6, Rank::R3];

    /// A plain quiet move, that might as well capture something.
    #[inline]
    pub const fn quiet(from: Square, to: Square) -> Move {
        Move{ from, to, flag: MoveFlag::Quiet }
    }

    /// An en passant move that captures a pawn at a square.
    #[inline]
    pub const fn en_passant(from: Square, to: Square, passed: Square) -> Move {
        Move{ from, to, flag: MoveFlag::EnPassant(passed) }
    }

    /// A promotion into a piece type.
    /// 
    /// #Panics
    /// 
    /// When `ptype` is not adequate.
    #[inline]
    pub fn promotion(from: Square, to: Square, ptype: PieceType) -> Move {
        if !ptype.can_be_promotion() {
           panic!("Inadequate piece type for promotion: {}", ptype)
        }
        Move{ from, to, flag: MoveFlag::Promotion(ptype) }
    }

    /// Make a castling for a player and a side.
    /// 
    /// ```
    /// use chess_std::{Color, Square, Move, Side};
    /// 
    /// let mv = Move::castling(Color::Black, Side::Queen);
    /// assert!(mv.from == Square::E8 && mv.to == Square::B8);
    /// ```
    #[inline]
    pub fn castling(col: Color, side: castling::Side) -> Move {
        let (from, to) = Self::castling_coords(col, side, King);
        Move { from, to, flag: Castling(side) }
    }

    // Get the origin and the destination of a `half` castling move,
    // either from the king or the rook.
    #[inline]
    pub(crate) fn castling_coords(col: Color, side: castling::Side,
                                   ptype: PieceType) -> (Square, Square) {
        let i = match ptype {
            King => 0,
            Rook => 1,
            _    => panic!("Invalid piece type for castling: {}", ptype)
        };
        Self::CASTLINGS[col.index()][side as usize][i]
    }

    /// An utility function to get the movement of the rook when castling.
    #[inline]
    pub fn rook_castling_coords(col: Color, side: Side) -> (Square, Square) {
        Self::castling_coords(col, side, Rook)
    }

    /// Whether the move is null.
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == Self::NONE
    }

    /// A simple verification of double push nature.
    /// ```
    /// use chess_std::{Color, Square, Move};
    /// assert!(Move::quiet(Square::D2, Square::D4).is_double_push(Color::White));
    /// ```
    #[inline]
    pub fn is_double_push(&self, col: Color) -> bool {
        use crate::units::Direction;
        let dir = Direction::of_pawns(col);
        Rank::R2.relative(col) == self.from.rank() &&
        self.from.shift(dir).shift(dir) == self.to
    }

    /// A fast sanity check, which does not take in account the position.
    pub fn is_valid(&self, col: Color) -> bool {
        if !(self.from != self.to &&
             self.from.is_on_board() && self.to.is_on_board()) {
            return false;
        }
        match self.flag {
            EnPassant(_) =>
                self.to.rank() == Self::EN_PASSANT_RANKS[col.index()],
            Promotion(ptype) =>
                Rank::last(col) == self.to.rank() && ptype.can_be_promotion(),
            Castling(side) => {
                Self::castling(col, side) == *self
            },
            _ => true
        }
    }
}

use std::fmt;

impl fmt::Display for Move {
    fn fmt(&self, ft: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(ft, "Move({}, {}, {:?})",
            self.from.san(),
            self.to.san(),
            self.flag
        )?;
        Ok(())
    }
}



/// The status of king, whether it may be check or even checkmate.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CheckType {
    None,
    Check,
    Checkmate
}

impl fmt::Display for CheckType {
    fn fmt(&self, ft: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(ft, "{}", match self {
            CheckType::None      => "",
            CheckType::Check     => "+",
            CheckType::Checkmate => "#"
        })?;
        Ok(())
    }
}

/// A more complete type that stores the piece moved,
/// the capture of the move and eventual check/checkmate.
/// It does not supports annotations though.
#[cfg(feature = "pgn")]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct PGNMove {
    pub ptype: PieceType,
    pub from: Square,
    pub to: Square,
    pub capture: Option<PieceType>,
    pub flag: MoveFlag,
    pub check: CheckType
}

#[cfg(feature = "pgn")]
impl PGNMove {
    /// Extend a normal move with additional data.
    pub fn from_plain(mv: Move, ptype: PieceType,
                      capture: Option<PieceType>, check: CheckType) -> PGNMove {
        PGNMove{
            ptype,
            from: mv.from, 
            to: mv.to,
            capture,
            flag: mv.flag,
            check
        }
    }
}

#[cfg(feature = "pgn")]
impl From<PGNMove> for Move {
    fn from(pgn_mv: PGNMove) -> Move {
        Move {
            from: pgn_mv.from,
            to:   pgn_mv.to,
            flag: pgn_mv.flag
        }
    }
}

/// The long SAN notation.
#[cfg(feature = "pgn")]
impl fmt::Display for PGNMove {
    fn fmt(&self, ft: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (from_s, to_s) = (self.from.san(), self.to.san());
        let pc_s = if self.ptype != Pawn {
            self.ptype.to_char().to_string()
        } else {
            String::new()
        };
        let cap_s = if self.capture.is_some() { "x" } else { "" };
        write!(
            ft, "{}{}",
            match self.flag {
                Quiet =>
                    format!("{}{}{}{}", pc_s, self.from.san(), cap_s, to_s),
                EnPassant(_) =>
                    format!("{}x{}e.p.", from_s, to_s),
                Promotion(new) =>
                    format!("{}{}{}={}", from_s, cap_s, to_s, new.to_char()),
                Castling(side) => match side {
                    Side::King  => "O-O".to_owned(),
                    Side::Queen => "O-O-O".to_owned(),
                },
            },
            self.check
        )?;
        Ok(())
    }
}