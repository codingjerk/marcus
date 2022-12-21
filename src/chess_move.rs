use std::fmt;

use crate::prelude::*;

pub type MoveInner = u32; // PERF: try smaller and bigger types

// Bit structure:
// - - - - - - - - - - - - - - - - - - -
// _ \___/ \___/ [   from  ] [   to    ]
// ^   ^     ^
// |   |     | - captured piece
// |   |
// |   | - promoted piece
// | - special bit
// Total bits: 1 + 3 + 3 + 6 + 6 = 19
// PERF: promoted piece could be encoded in special
//       bits to save space cause it's impossible
//       to promote to king or pawn
#[derive(Copy, Clone, PartialEq)]
pub struct Move(MoveInner);

impl Move {
    pub const Mask: MoveInner = 0b1111111111111111111;

    // PERF: try to store Piece instead of Dignity
    pub const fn new(
        from: Square,
        to: Square,
        captured: Dignity,
        promoted: Dignity,
        special_bits: MoveInner,
    ) -> Self {
        let bits =
            (from.index() as MoveInner)
            ^ ((to.index() as MoveInner) << 6)
            ^ ((captured.index() as MoveInner) << 12)
            ^ ((promoted.index() as MoveInner) << 15)
            // TODO: refactor special bits
            ^ special_bits << 18
        ;

        unsafe { always(bits & Self::Mask == bits) }

        Self(bits)
    }

    pub const fn capture(from: Square, to: Square, captured: Dignity) -> Self {
        // TODO: check captured is not None

        Self::new(from, to, captured, DignityNone, 0)
    }

    pub const fn quiet(from: Square, to: Square) -> Self {
        Self::new(from, to, DignityNone, DignityNone, 0)
    }

    pub const fn pawn_single(from: Square, to: Square) -> Self {
        // TODO: check from and to squares

        Self::new(from, to, DignityNone, DignityNone, 0)
    }

    pub const fn pawn_double(from: Square, to: Square) -> Self {
        // TODO: check from and to squares

        Self::new(from, to, DignityNone, DignityNone, 0)
    }

    pub const fn en_passant(from: Square, to: Square) -> Self {
        // TODO: check from and to squares

        Self::new(from, to, Pawn, DignityNone, 1)
    }

    pub const fn promotion(from: Square, to: Square, promoted: Dignity) -> Self {
        // TODO: check from and to squares

        Self::new(from, to, DignityNone, promoted, 0)
    }

    pub const fn promotion_capture(
        from: Square,
        to: Square,
        captured: Dignity,
        promoted: Dignity,
    ) -> Self {
        // TODO: check from and to squares

        Self::new(from, to, captured, promoted, 0)
    }

    pub fn king_side_castle(
        from: Square,
        to: Square,
    ) -> Self {
        unsafe {
            always(from.file() == FileE);
            always(to.file() == FileG);
        }

        Self::new(from, to, DignityNone, DignityNone, 0)
    }

    pub fn queen_side_castle(
        from: Square,
        to: Square,
    ) -> Self {
        unsafe {
            always(from.file() == FileE);
            always(to.file() == FileC);
        }

        Self::new(from, to, DignityNone, DignityNone, 0)
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

    // TODO: find a way to make these functions const
    // TODO: consider passing moved dignity here
    //       to make sure it returns true only for king moves
    //       current implementation is too lose
    pub fn is_pawn_double_move(self) -> bool {
        // NOTE: this function is only applicable for moves of pawn
        ( // White move
            (self.from().rank() == Rank2) &&
            (self.to().rank() == Rank4)
        ) || ( // Black move
            (self.from().rank() == Rank7) &&
            (self.to().rank() == Rank5)
        )
    }

    pub fn is_king_side_castle(self) -> bool {
        (self.from().file() == FileE) &&
        (self.to().file() == FileG)
    }

    pub fn is_queen_side_castle(self) -> bool {
        (self.from().file() == FileE) &&
        (self.to().file() == FileC)
    }

    pub fn is_en_passant(self) -> bool {
        // TODO: refactor
        self.index() & (1 << 18) != 0
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ff, fr) = self.from().fen();
        let (tf, tr) = self.to().fen();
        let bytes = [ff, fr, tf, tr];

        write!(f, "{}", unsafe {
            std::str::from_utf8_unchecked(&bytes)
        })?;

        let promoted = self.promoted();
        if promoted != DignityNone {
            write!(f, "{}", promoted.as_char())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_stucture() {
        let chess_move = Move::capture(a2, a3, Pawn);

        assert_eq!(a2, chess_move.from());
        assert_eq!(a3, chess_move.to());
        assert_eq!(Pawn, chess_move.captured());
    }

    #[test]
    fn format_promotion() {
        let chess_move = Move::promotion(a7, a8, Queen);

        assert_eq!(format!("{:?}", chess_move), "a7a8Q");
    }

    #[test]
    fn format_capture() {
        let chess_move = Move::capture(e4, f5, Rook);

        assert_eq!(format!("{:?}", chess_move), "e4f5");
    }
}
