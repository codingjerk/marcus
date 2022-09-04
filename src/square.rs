use crate::prelude::*;

pub type SquareInner = u8; // PERF: try smaller and bigger types

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Square(SquareInner);

impl Square {
    pub const Mask: SquareInner = 0b111111;

    pub const fn from_index(index: SquareInner) -> Self {
        unsafe { always(index & Self::Mask == index) }

        Self(index)
    }

    pub const fn index(self) -> SquareInner {
        self.0
    }
}

pub const A1: Square = Square(0);
pub const A2: Square = Square(1);
pub const A3: Square = Square(2);
