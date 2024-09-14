use wasm_bindgen::prelude::*;
use derive_more::{Add, Sub};

use chess_std as cs;

use std::convert::TryFrom;

/// The designation of a player.
#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Color(pub (crate) cs::Color);


#[wasm_bindgen]
impl Color {

    pub fn copy(&self) -> Self {
        Self(self.0)
    }

    pub fn equals(&self, rhs: &Color) -> bool {
        *self == *rhs
    }

    #[wasm_bindgen(getter)]
    pub fn opponent(&self) -> Color {
        Self(self.0.opponent())
    }

    /// Parse a color from a char.
    pub fn fromChar(c: char) -> Result<Color, JsValue> {
        cs::Color::try_from(c).map(Self)
            .map_err(|_| js_sys::Error::new("Couldn't parse color").into())
    }

    pub fn toString(&self) -> char {
        self.0.to_char()
    }
}



/// The role of a piece, which determines its moves.
#[wasm_bindgen]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct PieceType(pub (crate) cs::PieceType);

#[wasm_bindgen]
impl PieceType {

    pub fn copy(&self) -> Self {
        Self(self.0)
    }

    pub fn equals(&self, rhs: &PieceType) -> bool {
        *self == *rhs
    }

    /// The relative piece value.
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> u8 {
        self.0.value()
    }

    /// If a pawn can promote into this piece type.
    pub fn canBePromotion(&self) -> bool {
        self.0.can_be_promotion()
    }

    /// Parse a color from a char.
    pub fn fromChar(c: char) -> Result<PieceType, JsValue> {
        cs::PieceType::try_from(c).map(Self)
            .map_err(|_| js_sys::Error::new("Couldn't parse piece type").into())
    }

    pub fn toString(&self) -> char {
        self.0.to_char()
    }
}

/// A Piece is owned by a player and has a type.
#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Piece(pub (crate) cs::Piece);

#[wasm_bindgen]
impl Piece {

    pub fn copy(&self) -> Self {
        Self(self.0)
    }

    pub fn equals(&self, rhs: &Piece) -> bool {
        *self == *rhs
    }

    #[wasm_bindgen(constructor)]
    pub fn new(color: &Color, ptype: &PieceType) -> Self {
        Self(cs::Piece{ color: color.0, ptype: ptype.0 })
    }

    #[wasm_bindgen(getter)]
    pub fn color(&self) -> Color {
        Color(self.0.color)
    }

    #[wasm_bindgen(method, setter)]
    pub fn set_color(&mut self, col: &Color) {
        self.0.color = col.0;
    }

    #[wasm_bindgen(getter)]
    pub fn ptype(&self) -> PieceType {
        PieceType(self.0.ptype)
    }

    #[wasm_bindgen(method, setter)]
    pub fn set_ptype(&mut self, ptype: &PieceType) {
        self.0.ptype = ptype.0;
    }

    pub fn toString(&self) -> char {
        self.0.to_char()
    }

    pub fn symbol(&self) -> char {
        self.0.symbol()
    }

    #[wasm_bindgen(catch)]
    pub fn fromChar(c: char) -> Result<Piece, JsValue> {
        cs::Piece::try_from(c)
            .map(Self)
            .map_err(|_| js_sys::Error::new("Couldn't parse piece").into())
    }
}


type Rank = u8;
type File = u8;

#[wasm_bindgen]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Add, Sub, Hash)]
pub struct Square(pub u8);

#[wasm_bindgen]
impl Square {
    pub (crate) fn cs(&self) -> cs::Square {
       cs::Square::from(self.0)
    }

    pub (crate) fn from_cs(sq: cs::Square) -> Self {
        Self(sq.into())
    }

    pub fn copy(&self) -> Self {
        Self(self.0)
    }

    pub fn equals(&self, rhs: &Square) -> bool {
        *self == *rhs
    }

    #[wasm_bindgen(constructor)]
    pub fn new(rank: u8, file: u8) -> Self {
        let sq = cs::Square::new(cs::Rank::from(rank), cs::File::from(file));
        Self(sq.into())
    }

    pub fn fromScalar(i: u8) -> Self {
        Self(i)
    }

    pub fn toScalar(&self) -> u8 {
        self.0
    }

    #[wasm_bindgen(getter)]
    pub fn rank(&self) -> Rank {
        self.cs().rank().into()
    }

    #[wasm_bindgen(getter)]
    pub fn file(&self) -> File {
        self.cs().file().into()
    }

    pub fn isOnBoard(&self) -> bool {
        self.cs().is_on_board()
    }

    /// Returns whether the color of the square on the chessboard is dark (brown).
    pub fn isDark(&self) -> bool {
        self.cs().is_dark()
    }

    /// Returns the lowercase SAN notation of a square.
    pub fn san(&self) -> String {
        self.cs().san()
    }

    /// Creates a square from SAN notation, either lowercase or uppercase.
    #[wasm_bindgen(catch)]
    pub fn fromSan(san: &str) -> Result<Square, JsValue> {
        cs::Square::from_san(san)
            .map(Self::from_cs)
            .map_err(|_| js_sys::Error::new("Couldn't parse SAN").into())
    }
}
