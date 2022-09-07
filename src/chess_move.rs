use crate::prelude::*;

pub type MoveInner = u32; // PERF: try smaller and bigger types

// Bit structure:
// - - - - - - - - - - - - -
//   ^   [  from ] [  to   ]
//   | - captured piece
// Total bits: 5 + 5 + 3 = 13
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move(MoveInner);

impl Move {
    pub const Mask: MoveInner = 0b111111111111111;

    // PERF: try to store Piece instead of Dignity
    pub const fn new(from: Square, to: Square, captured: Dignity) -> Self {
        let bits =
            (from.index() as MoveInner)
            ^ ((to.index() as MoveInner) << 6)
            ^ ((captured.index() as MoveInner) << 12)
        ;

        unsafe { always(bits & Self::Mask == bits) }

        Self(bits)
    }

    pub const fn capture(from: Square, to: Square, captured: Dignity) -> Self {
        Self::new(from, to, captured)
    }

    pub const fn quiet(from: Square, to: Square) -> Self {
        Self::new(from, to, DignityNone)
    }

    pub const fn pawn_single(from: Square, to: Square) -> Self {
        Self::new(from, to, DignityNone)
    }

    pub const fn pawn_double(from: Square, to: Square) -> Self {
        Self::new(from, to, DignityNone)
    }

    pub const fn from(self) -> Square {
        let index = (self.0 as SquareInner) & Square::Mask;

        Square::from_index(index)
    }

    pub const fn to(self) -> Square {
        let index = ((self.0 >> 6) as SquareInner) & Square::Mask;

        Square::from_index(index)
    }

    pub const fn captured(self) -> Dignity {
        let index = ((self.0 >> 12) as DignityInner) & Dignity::Mask;

        Dignity::from_index(index)
    }

    pub const fn index(self) -> MoveInner {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_magic() {
        let chess_move = Move::capture(a2, a3, DignityNone);

        assert_eq!(a2, chess_move.from());
        assert_eq!(a3, chess_move.to());
        assert_eq!(DignityNone, chess_move.captured());
    }
}
