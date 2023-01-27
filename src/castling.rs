use crate::prelude::*;

pub type CastlingRightsInner = u8; // PERF: try smaller and bigger types

// Bit structure:
// - - - -
// ^ ^ ^ ^ - BlackQueenSide
// | | | - BlackKingSide
// | | - WhiteQueenSide
// | - WhiteKingSide
// Total bits: 4
#[derive(Copy, Debug, Eq)]
#[derive_const(Clone, PartialEq)]
pub struct CastlingRights(CastlingRightsInner);

impl CastlingRights {
    pub const Mask: CastlingRightsInner = 0b1;

    #[inline(always)]
    pub const fn from_index(index: CastlingRightsInner) -> Self {
        always!(index & Self::Mask == index);

        Self(index)
    }

    // PERF (better): make it templated by Color
    // PERF: make it simple shl by color (by separate direction + wbwb layout)
    #[inline(always)]
    pub const fn king_side(side_to_move: Color) -> Self {
        match side_to_move {
            Black => BlackKingSide,
            White => WhiteKingSide,
            _ => never!(),
        }
    }

    #[inline(always)]
    pub const fn queen_side(side_to_move: Color) -> Self {
        match side_to_move {
            Black => BlackQueenSide,
            White => WhiteQueenSide,
            _ => never!(),
        }
    }

    #[inline(always)]
    pub const fn both(side_to_move: Color) -> Self {
        match side_to_move {
            Black => CastlingRightsBlack,
            White => CastlingRightsWhite,
            _ => never!(),
        }
    }

    #[inline(always)]
    pub const fn index(self) -> CastlingRightsInner {
        self.0
    }

    #[inline(always)]
    pub fn unset(&mut self) {
        self.0 = 0;
    }

    // TODO: rename to something like `allow_from_fen`, name other methods accordingly
    #[inline(always)]
    pub fn set_from_fen(&mut self, fen: u8) {
        const FEN_TO_CASTLING: [CastlingRights; b'q' as usize + 1] = {
            let mut xs = [CastlingRightsNone; _];

            xs[b'K' as usize] = WhiteKingSide;
            xs[b'Q' as usize] = WhiteQueenSide;
            xs[b'k' as usize] = BlackKingSide;
            xs[b'q' as usize] = BlackQueenSide;

            xs
        };

        let castling = get_unchecked!(FEN_TO_CASTLING, fen);
        always!(castling != CastlingRightsNone);

        self.allow(castling);
    }

    #[inline(always)]
    pub const fn is_allowed(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[inline(always)]
    pub fn allow(&mut self, other: Self) {
        self.0 |= other.0;
    }

    #[inline(always)]
    pub fn disallow(&mut self, other: Self) {
        self.0 &= !other.0;
    }

    #[inline(always)]
    pub fn fen(self, buffer: &mut FenBuffer) {
        if self == CastlingRightsNone {
            buffer.add(b'-');
            return;
        }

        if self.is_allowed(WhiteKingSide) {
            buffer.add(b'K');
        }

        if self.is_allowed(WhiteQueenSide) {
            buffer.add(b'Q');
        }

        if self.is_allowed(BlackKingSide) {
            buffer.add(b'k');
        }

        if self.is_allowed(BlackQueenSide) {
            buffer.add(b'q');
        }
    }

    #[inline(always)]
    pub const fn king_destination(self) -> Square {
        match self {
            BlackQueenSide => c8,
            BlackKingSide  => g8,
            WhiteQueenSide => c1,
            WhiteKingSide  => g1,
            _ => never!(),
        }
    }

    #[inline(always)]
    pub const fn rook_initial(self) -> Square {
        match self {
            BlackQueenSide => a8,
            BlackKingSide  => h8,
            WhiteQueenSide => a1,
            WhiteKingSide  => h1,
            _ => never!(),
        }
    }

    #[inline(always)]
    pub const fn rook_destination(self) -> Square {
        match self {
            BlackQueenSide => d8,
            BlackKingSide  => f8,
            WhiteQueenSide => d1,
            WhiteKingSide  => f1,
            _ => never!(),
        }
    }
}

pub const CastlingRightsNone: CastlingRights = CastlingRights(0b0000);

pub const BlackQueenSide: CastlingRights = CastlingRights(0b0001);
pub const BlackKingSide: CastlingRights  = CastlingRights(0b0010);
pub const WhiteQueenSide: CastlingRights = CastlingRights(0b0100);
pub const WhiteKingSide: CastlingRights  = CastlingRights(0b1000);

pub const CastlingRightsBlack: CastlingRights = CastlingRights(0b0011);
pub const CastlingRightsWhite: CastlingRights = CastlingRights(0b1100);

pub const CastlingRightsAll: CastlingRights = CastlingRights(0b1111);
