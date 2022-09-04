use crate::prelude::*;

pub type ColorInner = u8; // PERF: try smaller and bigger types

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(ColorInner);

impl Color {
    pub const Mask: ColorInner = 0b1;

    pub const fn from_index(index: ColorInner) -> Self {
        unsafe { always(index & Self::Mask == index) }

        Self(index)
    }

    pub const fn index(self) -> ColorInner {
        self.0
    }
}

pub const Black: Color = Color(0);
pub const White: Color = Color(1);
