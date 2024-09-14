use wasm_bindgen::prelude::*;

use chess_std as cs;
use crate::units::{Square, Color, PieceType};


/// The side of a castling.
#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CastlingSide {
    King  = 0,
    Queen = 1
}

impl CastlingSide {
    pub (crate) fn cs(&self) -> cs::Side {
        match self {
            CastlingSide::King  => cs::Side::King,
            CastlingSide::Queen => cs::Side::Queen
        }
    }

    pub (crate) fn from_cs(side: cs::Side) -> Self {
        match side {
            cs::Side::King  => CastlingSide::King,
            cs::Side::Queen => CastlingSide::Queen
        }
    }
}

/// A tuple of two squares.
#[wasm_bindgen]
pub struct SquareVector(pub Square, pub Square);

/// A minimal move information.
#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    flag: cs::MoveFlag
}

#[wasm_bindgen]
impl Move {
    pub (crate) fn cs(&self) -> cs::Move {
        cs::Move { from: self.from.cs(), to: self.to.cs(), flag: self.flag }
    }

    pub (crate) fn from_cs(mv: cs::Move) -> Self {
        Move {
            from: Square::from_cs(mv.from),
            to: Square::from_cs(mv.to),
            flag: mv.flag
        }
    }

    pub fn equals(&self, other: &Move) -> bool {
        *self == *other
    }

    pub fn copy(&self) -> Self {
        *self
    }

    /// A plain quiet move, that might as well capture something.
    pub fn quiet(from: &Square, to: &Square) -> Self {
        Move{ from: *from, to: *to, flag: cs::MoveFlag::Quiet }
    }

    /// An en passant move, knowing the square of the passed pawn.
    pub fn enPassant(from: &Square, to: &Square, passed: &Square) -> Self {
        Move { from: *from, to: *to, flag: cs::EnPassant(passed.cs()) }
    }

    pub fn promotion(from: &Square, to: &Square, prom: &PieceType) -> Self {
        Move { from: *from, to: *to, flag: cs::Promotion(prom.0) }
    }

    /// Make a castling.
    pub fn castling(col: Color, side: CastlingSide) -> Self {
        let mv = cs::Move::castling(col.0, side.cs());
        Move {
            from: Square::from_cs(mv.from),
            to: Square::from_cs(mv.to),
            flag: mv.flag
        }
    }



    pub fn isQuiet(&self) -> bool {
        matches!(self.flag, cs::Quiet)
    }

    pub fn isEnPassant(&self) -> bool {
        matches!(self.flag, cs::EnPassant(_))
    }

    #[wasm_bindgen(getter)]
    pub fn passedSquare(&self) -> Square {
        if let cs::EnPassant(sq) = self.flag {
            Square::from_cs(sq)
        } else {
            panic!("Move is not en passant")
        }
    }

    /// A helper function to know the 
    pub fn rookCastlingVector(&self, col: &Color) -> SquareVector {
        if let cs::Castling(side) = self.cs().flag {
            let (from, to) = cs::Move::rook_castling_coords(col.0, side);
            SquareVector(Square::from_cs(from), Square::from_cs(to))
        } else {
            panic!("Not a castling move")
        }
    }

    pub fn isPromotion(&self) -> bool {
        matches!(self.flag, cs::Promotion(_))
    }

    #[wasm_bindgen(getter)]
    pub fn promotedInto(&self) -> PieceType {
        if let cs::Promotion(ptype) = self.flag {
            PieceType(ptype)
        } else {
            panic!("Move is not promotion")
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_promotedInto(&mut self, ptype: &PieceType) {
        if let cs::Promotion(_) = self.flag {
            if ptype.0.can_be_promotion() {
                self.flag = cs::Promotion(ptype.0);
            } else {
                panic!("Cannot set promotion to {}", ptype.0);
            }
        } else {
            panic!("Move is not promotion")
        }
    }

    pub fn isCastling(&self) -> bool {
        matches!(self.flag, cs::Castling(_))
    }

    #[wasm_bindgen(getter)]
    pub fn castlingSide(&self) -> CastlingSide {
        if let cs::Castling(side) = self.flag {
            CastlingSide::from_cs(side)
        } else {
            panic!("Move is not castling")
        }
    }

    pub fn toString(&self) -> String {
        format!("{}", self.cs())
    }
}


pub fn slice_into_array(gen: &[cs::Move]) -> js_sys::Array {
    gen.iter().map(|mv| JsValue::from(Move::from_cs(*mv))).collect()
}

pub fn gen_into_array(gen: impl cs::MoveGenerator) -> js_sys::Array {
    gen.map(|mv| JsValue::from(Move::from_cs(mv))).collect()
}


#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum CheckType {
    None,
    Check,
    Mate
}

impl CheckType {
    pub (crate) fn from_cs(ct: cs::CheckType) -> Self {
        match ct {
            cs::CheckType::None => Self::None,
            cs::CheckType::Check => Self::Check,
            cs::CheckType::Checkmate => Self::Mate
        }
    }
}


/// A more complete type that stores the piece moved,
/// the capture of the move and eventual check/checkmate.
/// It does not supports annotations though.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct PGNMove(pub (crate) cs::PGNMove);

#[wasm_bindgen]
impl PGNMove {

    pub fn copy(&self) -> Self {
        Self(self.0)
    }

    pub fn equals(&self, other: &PGNMove) -> bool {
        self.0 == other.0
    }

    #[wasm_bindgen(getter)]
    pub fn from(&self) -> Square {
        Square::from_cs(self.0.from)
    }

    #[wasm_bindgen(getter)]
    pub fn to(&self) -> Square {
        Square::from_cs(self.0.to)
    }

    #[wasm_bindgen(getter)]
    pub fn ptype(&self) -> PieceType {
        PieceType(self.0.ptype)
    }

    #[wasm_bindgen(getter)]
    pub fn capture(&self) -> Option<PieceType> {
        self.0.capture.map(PieceType)
    }

    #[wasm_bindgen(getter)]
    pub fn check(&self) -> CheckType {
        CheckType::from_cs(self.0.check)
    }


    pub fn isQuiet(&self) -> bool {
        matches!(self.0.flag, cs::Quiet)
    }

    pub fn isEnPassant(&self) -> bool {
        matches!(self.0.flag, cs::EnPassant(_))
    }

    #[wasm_bindgen(getter)]
    pub fn passedSquare(&self) -> Square {
        if let cs::EnPassant(sq) = self.0.flag {
            Square::from_cs(sq)
        } else {
            panic!("Move is not en passant")
        }
    }

    pub fn isPromotion(&self) -> bool {
        matches!(self.0.flag, cs::Promotion(_))
    }

    #[wasm_bindgen(getter)]
    pub fn promotedInto(&self) -> PieceType {
        if let cs::Promotion(ptype) = self.0.flag {
            PieceType(ptype)
        } else {
            panic!("Move is not promotion")
        }
    }

    pub fn isCastling(&self) -> bool {
        matches!(self.0.flag, cs::Castling(_))
    }

    #[wasm_bindgen(getter)]
    pub fn castlingSide(&self) -> CastlingSide {
        if let cs::Castling(side) = self.0.flag {
            CastlingSide::from_cs(side)
        } else {
            panic!("Move is not castling")
        }
    }

    pub fn toString(&self) -> String {
        format!("{}", self.0)
    }
}