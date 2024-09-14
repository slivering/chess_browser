
use crate::prelude::*;
use crate::bit::{self, Bitboard};
use crate::attack::{fill_line};
use crate::position::{Board};

use arrayvec::ArrayVec;


/// A container to store a bitboard of moves from a piece (knowing its square).
/// It cannot store en passant or castling moves.
#[derive(Clone)]
struct MovesFromSquare {
    moves: Bitboard,
    from: Square,
}

impl MovesFromSquare {
    pub fn new(moves: Bitboard, from: Square) -> Self {
        MovesFromSquare{ moves, from }
    }
}


type QuietMoves = ArrayVec<MovesFromSquare, 16>; // May be quiets OR promotions.
type SpecialMoves = ArrayVec<Move, 4>;           // May be en passant or castlings.

pub trait MoveGenerator
    where Self: ExactSizeIterator<Item = Move>
{
    /// Whether a move would be yielded by the iterator,
    /// regardless of masks for `MoveGenMasked`.
    fn contains(&self, mv: Move) -> bool;
}

/// A container for efficient legal moves storage and iteration.
/// It allows to filter moves by a `Bitboard` mask.
#[derive(Clone)]
pub struct MoveGenMasked {
    quiets: QuietMoves,
    specials: SpecialMoves,
    orig_mask: Bitboard,
    dest_mask: Bitboard,
    promotion_mask: Bitboard,
    promotion_index: usize
}

impl MoveGenMasked {

    // Whether a move is covered by the masks.
    #[inline]
    pub fn is_covered(&self, mv: Move) -> bool {
        self.orig_mask.get(mv.from) && self.dest_mask.get(mv.to)
    }

    /// Restrict move iteration for given move origins.
    /// This does NOT prevent from storing moves uncovered by the mask.
    pub fn set_origin_mask(&mut self, froms: Bitboard) {
        self.orig_mask = froms;
    }

    /// Restrict move iteration for given move destinations.
    /// This does NOT prevent from storing moves uncovered by the mask.
    pub fn set_destination_mask(&mut self, dests: Bitboard) {
        self.dest_mask = dests;
    }
}

impl Iterator for MoveGenMasked {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(of_piece) = self.quiets.first_mut() {
            if !self.orig_mask.get(of_piece.from) {
                // Skip the moves from here
                self.quiets.remove(0);
                self.next()
            } else if of_piece.moves.is_populated() {
                // The first destination from the piece
                let to = of_piece.moves.scan_forward();
                if !self.dest_mask.get(to) {
                    // Skip the move that lands here
                    of_piece.moves.remove(to);
                    self.next()
                } else if self.promotion_mask.get(of_piece.from) {
                    // Enumerate all the promotions from this piece
                    self.promotion_index += 1;
                    if self.promotion_index >= 4 {
                        // No more promotion in this destination, get another
                        self.promotion_index = 0;
                        of_piece.moves.remove(to);
                        self.next()
                    } else {
                        let ptype = ALL_PIECE_TYPES[self.promotion_index + 1];
                        Some(Move::promotion(of_piece.from, to, ptype))
                    }
                } else {
                    // Visited this destination
                    of_piece.moves.remove(to);
                    Some(Move::quiet(of_piece.from, to))
                }
            } else {
                // No more moves from this piece, remove first
                self.quiets.remove(0);
                self.next()
            }
        } else {
            // No more quiet moves
            let specials = self.specials.pop();
            specials.and_then(|mv| if self.is_covered(mv) { Some(mv) } else { None })
        }
    }
}

impl ExactSizeIterator for MoveGenMasked {
    
    fn len(&self) -> usize {
        let mut n: usize = 0;
        for of_piece in &self.quiets {
            if self.orig_mask.get(of_piece.from) {
                n += (of_piece.moves & self.dest_mask).pop_count() as usize
                    * if self.promotion_mask.get(of_piece.from) { 4 } else { 1 };
            }
        }
        n += self.specials.iter().filter(|mv| self.is_covered(**mv)).count();
        n
    }
}

impl MoveGenerator for MoveGenMasked {

    fn contains(&self, mv: Move) -> bool {
        if let Quiet = mv.flag {
            self.quiets.iter()
                .any(|of_pc| of_pc.from == mv.from && of_pc.moves.get(mv.to))
        } else {
            self.specials.iter()
                .any(|mv2| *mv2 == mv)
        }
    }
}

impl From<MoveGen> for MoveGenMasked {
    fn from(gen: MoveGen) -> Self {
        Self {
            quiets: gen.quiets,
            specials: gen.specials,
            orig_mask: bit::FULL,
            dest_mask: bit::FULL,
            promotion_mask: gen.promotion_mask,
            promotion_index: gen.promotion_index
        }
    }
}


/// A variant of MoveGenMasked, that does not allow move masks in favor of performance.
#[derive(Clone)]
pub struct MoveGen {
    quiets: QuietMoves,
    specials: SpecialMoves,
    promotion_mask: Bitboard,
    promotion_index: usize
}

impl MoveGen {

    pub(crate) fn new() -> Self {
        MoveGen {  
            quiets: QuietMoves::new(),
            specials: SpecialMoves::new(),
            promotion_mask: bit::EMPTY,
            promotion_index: 0
        }
    }

    /// Create a new generator from the legal moves of a board.
    pub fn new_from(board: &Board) -> Self {
        use crate::attack::*;

        let mut gen = Self::new();
        
        let from = board.king_square();
        let mut king_legals = bit::EMPTY;
        for to in of_king(from, board.own_color()) {
            if board.is_safe_to_move(from, to) {
                king_legals.add(to);
            }
        }
        gen.add_moves_from(from, king_legals);

        if  board.checkers.pop_count() < 2 {
            gen.add_non_king_moves(board);
            gen.add_castlings(board, board.king_square());
        }

        gen
    }

    // Add the moves from other pieces than the king.
    #[inline(always)]
    fn add_non_king_moves(&mut self, board: &Board) {
        use crate::attack::*;
        // The destinations squares where we can get out of check, if any.
        let mut dests = bit::FULL;
        if board.checkers.pop_count() == 1 {
            let checker = board.checkers.scan_forward();
            // by capture...
            dests = board.checkers;
            let sliders = board.opponent_piece_type(Bishop)
                        | board.opponent_piece_type(Rook)
                        | board.opponent_piece_type(Queen);
            if sliders.get(checker) {
                // ...or by obstructing the sliding piece.
                dests |= fill_between(board.king_square(), checker);
            }
        }
        let ours = board.own_color();
        let enemy = board.opponent_color();
        for from in board.own_piece_type(Pawn) {
            let attacks = (
                of_pawn(board.turn, from, enemy) |
                pawn_pushes(board.turn, from, ours | enemy))
                & dests;
            if from.rank() == Rank::R7.relative(board.turn) {
                self.add_non_outpinning_promotions(board, from, attacks);
            } else {
                self.add_non_outpinning_attacks(board, from, attacks);
            }
        }
        if let Some(sq) = board.ep_target {
            self.add_en_passant(board, sq, dests);
        }
        for from in board.own_piece_type(Knight) {
            let attacks = of_knight(from, ours) & dests;
            self.add_non_outpinning_attacks(board, from, attacks);
        }
        for from in board.own_piece_type(Bishop) {
            let attacks = of_bishop(from, ours, enemy) & dests;
            self.add_non_outpinning_attacks(board, from, attacks);
        }
        for from in board.own_piece_type(Rook) {
            let attacks = of_rook(from, ours, enemy) & dests;
            self.add_non_outpinning_attacks(board, from, attacks);
        }
        for from in board.own_piece_type(Queen) {
            let attacks = of_queen(from, ours, enemy) & dests;
            self.add_non_outpinning_attacks(board, from, attacks);
        }
    }

    // Try to add en passant moves for a target, if legal.
    #[inline(always)]
    fn add_en_passant(&mut self, board: &Board, ep_target: Square, dests: Bitboard) {
        use crate::Direction::{self, *};
        if !dests.get(ep_target) {
            return;
        }
        let passed = ep_target.shift(Direction::of_pawns(board.turn.opponent()));
        if board.is_pinned(passed) && board.king_square().file() != passed.file() {
            // The en passant capture is pinned on a rank/diagonal
            // and the capturer would leave the check by not being pinned.
            return;
        }
        // Convert to bitboard in case of edge overflowing
        let mut bb = bit::single(passed);
        bb = bb.shift(West) | bb.shift(East);
        for from in bb & board.own_piece_type(Pawn) {
            let pin_ray = fill_line(from, board.king_square());
            if !board.is_pinned(from) || pin_ray.get(ep_target) {
                // Does not outpin
                self.add_special_move(Move::en_passant(from, ep_target, passed));
            }
        }
    }

    // This will add castlings from king, the precondition is
    // having the king not checked.
    #[inline(always)]
    fn add_castlings(&mut self, board: &Board, king_sq: Square) {
        use crate::Direction::*;
        if board.in_check() {
            return;
        }
        if board.has_right(board.turn, Side::King) {
            let mv = Move::castling(board.turn, Side::King);
            let middle = king_sq.shift(East);
            let between = merge_sq!(middle, mv.to);
            if !board.occupied().intersects(between)
            && board.is_safe(middle, board.turn)
            && board.is_safe(mv.to,  board.turn) {
                self.add_special_move(mv);
            }
        }
        if board.has_right(board.turn, Side::Queen) {
            let mv = Move::castling(board.turn, Side::Queen);
            let middle = king_sq.shift(West);
            let between = merge_sq!(middle, mv.to, mv.to.shift(West));
            if !board.occupied().intersects(between)
            && board.is_safe(middle, board.turn)
            && board.is_safe(mv.to,  board.turn) {
                self.add_special_move(mv);
            }
        }
        
    }

    

    // Add all the attacks from a square, given they don't move a pinned piece
    // out of their pin direction.
    fn add_non_outpinning_attacks(&mut self, board: &Board,
                                  from: Square, mut attacks: Bitboard) {
        if board.is_pinned(from) {
            attacks &= fill_line(from, board.king_square());
        }
        if attacks.is_populated() {
            self.add_moves_from(from, attacks);
        }
    }

    fn add_non_outpinning_promotions(&mut self, board: &Board,
                                     from: Square, mut proms: Bitboard) {
        if board.is_pinned(from) {
            proms &= fill_line(from, board.king_square());
        }
        if proms.is_populated() {
            self.add_promotion_from(from, proms);
        }
    }



    /// Add normal (`Quiet`) moves.
    #[inline]
    pub(crate) fn add_moves_from(&mut self, from: Square, moves: Bitboard) {
        self.quiets.push(MovesFromSquare::new(moves, from));
    }

    /// Add an `EnPassant`, or `Castling` move.
    #[inline]
    pub(crate) fn add_special_move(&mut self, mv: Move) {
        self.specials.push(mv);
    }

    /// Add `Promotion` moves.
    #[inline]
    pub(crate) fn add_promotion_from(&mut self, from: Square, moves: Bitboard) {
        self.add_moves_from(from, moves);
        self.promotion_mask.add(from);
    }
}

impl Iterator for MoveGen {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(of_piece) = self.quiets.first_mut() {
            if of_piece.moves.is_populated() {
                // The first destination from the piece
                let to = of_piece.moves.scan_forward();
                if self.promotion_mask.get(of_piece.from) {
                    // Enumerate all the promotions from this piece
                    self.promotion_index += 1;
                    if self.promotion_index > 4 {
                        // No more promotion in this destination, get another
                        self.promotion_index = 0;
                        of_piece.moves.remove(to);
                        self.next()
                    } else {
                        let ptype = ALL_PIECE_TYPES[self.promotion_index];
                        Some(Move::promotion(of_piece.from, to, ptype))
                    }
                } else {
                    // Visited this destination
                    of_piece.moves.remove(to);
                    Some(Move::quiet(of_piece.from, to))
                }
            } else {
                // No more moves from this piece, remove first
                self.quiets.remove(0);
                self.next()
            }
        } else {
            // No more quiet moves
            self.specials.pop()
        }
    }
}

impl ExactSizeIterator for MoveGen {
    
    fn len(&self) -> usize {
        self.quiets
            .iter()
            .map(|of_piece| of_piece.moves.pop_count() *
                if self.promotion_mask.get(of_piece.from) { 4 } else { 1 })
            .sum::<u32>() as usize
            + self.specials.len()
    }
}

impl MoveGenerator for MoveGen {

    fn contains(&self, mv: Move) -> bool {
        match mv.flag {
            Quiet => self.quiets.iter()
                .any(|of_pc| of_pc.from == mv.from && of_pc.moves.get(mv.to)),
            Promotion(ptype) => ptype.can_be_promotion() && self.quiets.iter()
                .any(|of_pc| of_pc.from == mv.from && of_pc.moves.get(mv.to)),
            _ => self.specials.iter().any(|mv2| *mv2 == mv)
        }
    }
}


impl From<MoveGen> for Moves {
    fn from(gen: MoveGen) -> Self {
        let mut lst = gen.specials.to_vec();
        for of_piece in gen.quiets {
            for to in of_piece.moves {
                if gen.promotion_mask.get(of_piece.from) {
                    for ptype in &[Knight, Bishop, Rook, Queen] {
                        lst.push(Move::promotion(of_piece.from, to, *ptype));
                    }
                } else {
                    lst.push(Move::quiet(of_piece.from, to));
                }
            }
        }
        lst
    }
}