use std::fmt;

use crate::prelude::*;

pub type MoveInner = u32; // PERF: try smaller and bigger types

// Bit structure:
// - - - - - - - - - - - - - - - -
// \___/ \___/ [  from ] [  to   ]
//   ^     ^
//   |     | - captured piece
//   |
//   | - promoted piece
// Total bits: 3 + 3 + 5 + 5 = 16
// PERF: promoted piece could be encoded in special
//       bits to save space cause it's impossible
//       to promote to king or pawn
#[derive(Copy, Clone, PartialEq)]
pub struct Move(MoveInner);

impl Move {
    pub const Mask: MoveInner = 0b111111111111111111;

    // PERF: try to store Piece instead of Dignity
    pub const fn new(
        from: Square,
        to: Square,
        captured: Dignity,
        promoted: Dignity,
    ) -> Self {
        let bits =
            (from.index() as MoveInner)
            ^ ((to.index() as MoveInner) << 6)
            ^ ((captured.index() as MoveInner) << 12)
            ^ ((promoted.index() as MoveInner) << 15)
        ;

        unsafe { always(bits & Self::Mask == bits) }

        Self(bits)
    }

    pub const fn capture(from: Square, to: Square, captured: Dignity) -> Self {
        Self::new(from, to, captured, DignityNone)
    }

    pub const fn quiet(from: Square, to: Square) -> Self {
        Self::new(from, to, DignityNone, DignityNone)
    }

    pub const fn pawn_single(from: Square, to: Square) -> Self {
        Self::new(from, to, DignityNone, DignityNone)
    }

    pub const fn pawn_double(from: Square, to: Square) -> Self {
        Self::new(from, to, DignityNone, DignityNone)
    }

    pub const fn en_passant(from: Square, to: Square) -> Self {
        Self::new(from, to, Pawn, DignityNone)
    }

    pub const fn promotion(from: Square, to: Square, promoted: Dignity) -> Self {
        Self::new(from, to, DignityNone, promoted)
    }

    pub const fn promotion_capture(
        from: Square,
        to: Square,
        captured: Dignity,
        promoted: Dignity,
    ) -> Self {
        Self::new(from, to, captured, promoted)
    }

    pub fn king_side_castle(
        from: Square,
        to: Square,
    ) -> Self {
        unsafe {
            always(from.file() == FileE);
            always(to.file() == FileG);
        }

        Self::new(from, to, DignityNone, DignityNone)
    }

    pub fn queen_side_castle(
        from: Square,
        to: Square,
    ) -> Self {
        unsafe {
            always(from.file() == FileE);
            always(to.file() == FileC);
        }

        Self::new(from, to, DignityNone, DignityNone)
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

    pub const fn promoted(self) -> Dignity {
        let index = ((self.0 >> 15) as DignityInner) & Dignity::Mask;

        Dignity::from_index(index)
    }

    pub const fn index(self) -> MoveInner {
        self.0
    }

    pub fn is_king_side_castle(self) -> bool {
        (self.from().file() == FileE) &&
        (self.to().file() == FileG)
    }

    pub fn is_queen_side_castle(self) -> bool {
        (self.from().file() == FileE) &&
        (self.to().file() == FileC)
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ff, fr) = self.from().fen();
        let (tf, tr) = self.to().fen();
        let bytes = [ff, fr, tf, tr];

        // TODO: output promotion piece
        // TODO: create uci() function with same behavior
        //       and use it here
        write!(f, "{}", unsafe {
            std::str::from_utf8_unchecked(&bytes)
        })
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
