use crate::prelude::*;

pub type DignityInner = u8; // PERF: try smaller and bigger types

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dignity(DignityInner);

impl Dignity {
    pub const Mask: DignityInner = 0b111;

    pub const None: Self = Dignity(0);

    pub const fn from_index(index: DignityInner) -> Self {
        unsafe { always(index & Self::Mask == index) }

        Self(index)
    }

    pub const fn index(&self) -> DignityInner {
        self.0
    }
}

pub type PieceInner = u8; // PERF: try smaller and bigger types

// Bit structure:
// - - - -
// ^ [ - ] <- Dignity
// | - Color
// Total bits: 5 + 5 + 3 = 13
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece(PieceInner);

impl Piece {
    pub const Mask: PieceInner = 0b1111;

    pub const None: Self = Piece(0);

    pub const fn from_index(index: PieceInner) -> Self {
        unsafe { always(index & Self::Mask == index) }

        Self(index)
    }

    pub const fn index(self) -> PieceInner {
        self.0
    }

    pub const fn dignity(self) -> Dignity {
        let index = (self.0 as DignityInner) & Dignity::Mask;

        Dignity::from_index(index)
    }

    pub const fn color(self) -> Color {
        let index = ((self.0 >> 3) as ColorInner) & Color::Mask;

        Color::from_index(index)
    }
}
