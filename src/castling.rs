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

    pub const fn index(self) -> CastlingRightsInner {
        self.0
    }

    pub fn unset(&mut self) {
        self.0 = 0;
    }

    #[inline]
    pub fn set_from_fen(&mut self, fen: u8) {
        self.0 |= match fen {
            b'K' => WhiteKingSide.0,
            b'Q' => WhiteQueenSide.0,
            b'k' => BlackKingSide.0,
            b'q' => BlackQueenSide.0,

            _ => unsafe { unreachable() }
        };
    }

    pub const fn is_allowed(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
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
}

pub const CastlingRightsNone: CastlingRights = CastlingRights(0b0000);

pub const BlackQueenSide: CastlingRights = CastlingRights(0b0001);
pub const BlackKingSide: CastlingRights  = CastlingRights(0b0010);
pub const WhiteQueenSide: CastlingRights = CastlingRights(0b0100);
pub const WhiteKingSide: CastlingRights  = CastlingRights(0b1000);

pub const CastlingRightsAll: CastlingRights = CastlingRights(0b1111);
