use wasm_bindgen::prelude::*;

use chess_std as cs;

use crate::units::{Color, PieceType, Square};
use crate::moves::{self, Move, PGNMove};
use crate::position::Board;


#[wasm_bindgen]
impl Board {
    
    /// Get the half move clock.
    pub fn halfMoveClock(&self) -> u32 {
        self.0.half_move_clock
    }

    /// Returns the number of moves played since the beginning of the game.
    pub fn numMovesPlayed(&self) -> u32 {
        self.0.num_moves_played()
    }
  
    /// Returns all the legal moves on the board.
    pub fn legalMoves(&self) -> js_sys::Array {
        moves::gen_into_array(self.0.legal_moves())
    }

    /// Returns the legal moves of a piece at a square, using cache.
    pub fn legalMovesFrom(&self, sq: &Square) -> js_sys::Array {
        moves::gen_into_array(self.0.legal_moves_from(sq.cs()))
    }

    /// Returns the legal moves which are captures, using cache.
    pub fn legalCaptures(&self) -> js_sys::Array {
        moves::gen_into_array(self.0.legal_captures())
    }

    /// Returns the legal moves of a piece type, using cache.
    pub fn legalMovesOf(&self, ptype: &PieceType) -> js_sys::Array {
        moves::gen_into_array(self.0.legal_moves_of(ptype.0))
    }

    /// Returns the subsequent board after applying the move.
    /// This does not verify if the move is legal.
    pub fn playMove(&self, mv: &Move) -> Self {
        Self(self.0.play_move(mv.cs()))
    }

    /// The number of legal moves.
    pub fn numMoves(&self) -> usize {
        self.0.num_moves()
    }

    /// A move is legal if stored in cache. This function is implied to be fast.
    pub fn isMoveLegal(&mut self, mv: &Move) -> bool {
        self.0.is_move_legal(mv.cs())
    }

    /// Whether a move captures a piece.
    pub fn isMoveCapture(&mut self, mv: &Move) -> bool {
        self.0.captured_by(mv.cs()).is_some()
    }

    /// Extend a plain move with additional data as a PGN move.
    /// Keep in mind that this function is slow.
    pub fn pgnMove(&self, mv: &Move) -> PGNMove {
        PGNMove(self.0.pgn_move(mv.cs()))
    }

    /// If the current player's king is checked.
    pub fn inCheck(&self) -> bool {
        self.0.in_check()
    }

    /// If the current player's king is checkmated.
    pub fn inCheckmate(&self) -> bool {
        self.0.in_checkmate()
    }

    /// If the current player's king is stuck in stalemate.
    pub fn inStalemate(&self) -> bool {
        self.0.in_stalemate()
    }

    /// If the result is checkmate or stalemate.
    pub fn isFinished(&self) -> bool {
        self.0.is_finished()
    }

    /// If a draw type can be claimed, except ThreefoldRepetition.
    pub fn canClaimDrawWith(&self, dt: DrawType) -> bool {
        self.0.can_claim_draw_with(dt.cs())
    }

    /// When `this.get_result() is none`,
    /// however another draw might be claimed.
    /// 
    /// NOTE: use Game.can_claim_draw for threefold repetition.
    pub fn canClaimDraw(&self) -> bool {
        self.0.can_claim_draw()
    }

    /// Either the game is still ongoing, or a result can be declared.
    pub fn getResult(&self) -> GameResult {
        GameResult::from_cs(self.0.get_result())
    }

}



/// A win might be, other than checkmate, caused by resign.
#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum WinType {
    Resign,
    Checkmate
}

impl WinType {
    // pub (crate) fn cs(&self) -> cs::WinType {
    //     match self {
    //         WinType::Resign    => cs::WinType::Resign,
    //         WinType::Checkmate => cs::WinType::Checkmate
    //     }
    // }

    pub (crate) fn from_cs(wt: cs::WinType) -> Self {
        match wt {
            cs::WinType::Resign    => WinType::Resign,
            cs::WinType::Checkmate => WinType::Checkmate
        }
    }
}


/// A draw, other than stalemate, may be claimed by the player.
#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum DrawType {
    Agreement,
    Stalemate,
    ThreefoldRepetition,
    FiftyMoveRule,
    InsufficientMaterial
}

impl DrawType {
    pub (crate) fn cs(&self) -> cs::DrawType {
        use DrawType::*;
        use cs::DrawType as DT;
        match self {
            Agreement => DT::Agreement,
            Stalemate => DT::Stalemate,
            ThreefoldRepetition => DT::ThreefoldRepetition,
            FiftyMoveRule => DT::FiftyMoveRule,
            InsufficientMaterial => DT::InsufficientMaterial,
        }
    }

    pub (crate) fn from_cs(dt: cs::DrawType) -> Self {
        use DrawType::*;
        use cs::DrawType as DT;
        match dt {
            DT::Agreement => Agreement,
            DT::Stalemate => Stalemate,
            DT::ThreefoldRepetition => ThreefoldRepetition,
            DT::FiftyMoveRule => FiftyMoveRule,
            DT::InsufficientMaterial => InsufficientMaterial,
        }
    }
}


#[wasm_bindgen]
pub struct GameResult(cs::GameResult);

#[wasm_bindgen]
impl GameResult {
    
    pub (crate) fn from_cs(res: cs::GameResult) -> Self {
        Self(res)
    }

    pub fn copy(&self) -> Self {
        Self(self.0)
    }

    pub fn equals(&self, other: &GameResult) -> bool {
        self.0 == other.0
    }

    pub fn isUnfinished(&self) -> bool {
        matches!(self.0, cs::GameResult::NoResult)
    }

    #[wasm_bindgen(getter)]
    pub fn winner(&self) -> Option<Color> {
        match self.0 {
            cs::GameResult::Win(player, _) => Some(Color(player)),
            _ => None
        }
    }

    #[wasm_bindgen(getter)]
    pub fn winType(&self) -> WinType {
        if let cs::GameResult::Win(_, wt) = self.0 {
            WinType::from_cs(wt)
        } else {
            panic!("No win type")
        }
    }

    #[wasm_bindgen(getter)]
    pub fn drawType(&self) -> DrawType {
        if let cs::GameResult::Draw(dt) = self.0 {
            DrawType::from_cs(dt)
        } else {
            panic!("No draw type")
        }
    }
}

#[wasm_bindgen]
impl GameResult {
    pub fn toString(&self) -> String {
        format!("{}", self.0)
    }
}