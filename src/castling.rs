use crate::prelude::*;

pub type CastlingRightsInner = u8; // PERF: try smaller and bigger types

// Bit structure:
// - - - -
// ^ ^ ^ ^ - BlackQueenSide
// | | | - BlackKingSide
// | | - WhiteQueenSide
// | - WhiteKingSide
// Total bits: 4
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CastlingRights(CastlingRightsInner);

impl CastlingRights {
    pub const Mask: CastlingRightsInner = 0b1;

    pub const fn from_index(index: CastlingRightsInner) -> Self {
        unsafe { always(index & Self::Mask == index) }

        Self(index)
    }

    // TODO: use direction instead of different constructors
    // PERF: make it simple shl by color (by wbwb layout)
    pub const fn king_side(side_to_move: Color) -> Self {
        match side_to_move {
            Black => BlackKingSide,
            White => WhiteKingSide,
            _ => unsafe { unreachable() },
        }
    }

    pub const fn queen_side(side_to_move: Color) -> Self {
        match side_to_move {
            Black => BlackQueenSide,
            White => WhiteQueenSide,
            _ => unsafe { unreachable() },
        }
    }

    pub const fn index(self) -> CastlingRightsInner {
        self.0
    }

    pub fn unset(&mut self) {
        self.0 = 0;
    }

    #[inline]
    pub fn set_from_fen(&mut self, fen: u8) {
        const FEN_TO_CASTLING: [CastlingRights; b'q' as usize + 1] = {
            let mut xs = [CastlingRightsNone; _];

            xs[b'K' as usize] = WhiteKingSide;
            xs[b'Q' as usize] = WhiteQueenSide;
            xs[b'k' as usize] = BlackKingSide;
            xs[b'q' as usize] = BlackQueenSide;

            xs
        };

        let castling = unsafe { *FEN_TO_CASTLING.get_unchecked(fen as usize) };
        unsafe { always(castling != CastlingRightsNone) }

        self.allow(castling);
    }

    pub const fn is_allowed(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[inline]
    pub fn allow(&mut self, other: Self) {
        self.0 |= other.0;
    }

    #[inline]
    pub fn fen(self, buffer: &mut StaticBuffer<u8, 90>) {
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

    pub fn king_destination(self) -> Square {
        // TODO: use separate structures for single castling and
        //       castling "map"
        match self {
            cr if cr == BlackQueenSide => c8,
            cr if cr == BlackKingSide  => g8,
            cr if cr == WhiteQueenSide => c1,
            cr if cr == WhiteKingSide  => g1,
            _ => unsafe { unreachable() },
        }
    }

    pub fn rook_initial(self) -> Square {
        match self {
            cr if cr == BlackQueenSide => a8,
            cr if cr == BlackKingSide  => h8,
            cr if cr == WhiteQueenSide => a1,
            cr if cr == WhiteKingSide  => h1,
            _ => unsafe { unreachable() },
        }
    }

    pub fn rook_destination(self) -> Square {
        match self {
            cr if cr == BlackQueenSide => d8,
            cr if cr == BlackKingSide  => f8,
            cr if cr == WhiteQueenSide => d1,
            cr if cr == WhiteKingSide  => f1,
            _ => unsafe { unreachable() },
        }
    }
}

pub const CastlingRightsNone: CastlingRights = CastlingRights(0b0000);

pub const BlackQueenSide: CastlingRights = CastlingRights(0b0001);
pub const BlackKingSide: CastlingRights  = CastlingRights(0b0010);
pub const WhiteQueenSide: CastlingRights = CastlingRights(0b0100);
pub const WhiteKingSide: CastlingRights  = CastlingRights(0b1000);

pub const CastlingRightsAll: CastlingRights = CastlingRights(0b1111);
