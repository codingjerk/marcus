use crate::prelude::*;

pub type ColorInner = u8; // PERF: try smaller and bigger types

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Color(ColorInner);

impl Color {
    pub const Mask: ColorInner = 0b1;

    pub const fn from_index(index: ColorInner) -> Self {
        unsafe { always(index & Self::Mask == index) }

        Self(index)
    }

    #[inline(always)]
    pub fn from_fen(fen: u8) -> Self {
        unsafe { always(fen == b'b' || fen == b'w') }

        // NOTE: by happy coincidence
        //       code of 'b' ends with 0
        //       and code of 'w' ends with 1
        Self::from_index(fen & 0b1)
    }

    pub const fn index(self) -> ColorInner {
        self.0
    }

    #[inline]
    pub const fn start_rank(self) -> Rank {
        match self {
            Black => Rank8,
            White => Rank1,

            _ => unsafe { unreachable() },
        }
    }

    #[inline]
    pub const fn fen(self) -> u8 {
        match self {
            Black => b'b',
            White => b'w',

            _ => unsafe { unreachable() },
        }
    }

    pub fn swapped(self) -> Self {
        Self::from_index(self.0 ^ 0b1)
    }

    pub fn swap(&mut self) {
        *self = self.swapped();
    }
}

pub const Black: Color = Color(0);
pub const White: Color = Color(1);

#[cfg(test)]
mod bench {
    use super::*;

    use test::{Bencher, black_box};

    #[bench]
    fn from_fen(b: &mut Bencher) {
        b.iter(|| {
            let fen = black_box(b'w');
            black_box(Color::from_fen(fen))
        })
    }
}
