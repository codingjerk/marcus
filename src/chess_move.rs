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
    pub const Mask: MoveInner             = 0b1111111111111111111;
    pub const EnPassantSpecial: MoveInner = 0b1000000000000000000;

    // PERF: try to store Piece instead of Dignity
    #[inline(always)]
    pub const fn new(
        from: Square,
        to: Square,
        captured: Dignity,
        promoted: Dignity,
        special_bits: MoveInner,
    ) -> Self {
        let bits =
            (from.index() as MoveInner)               // 6 bits
            ^ ((to.index() as MoveInner) << 6)        // 6 bits
            ^ ((captured.index() as MoveInner) << 12) // 3 bits
            ^ ((promoted.index() as MoveInner) << 15) // 3 bits
            ^ special_bits << 18                      // 1 bit
        ;

        always!(bits & Self::Mask == bits);

        Self(bits)
    }

    #[inline(always)]
    pub const fn capture(from: Square, to: Square, captured: Dignity) -> Self {
        always!(captured != DignityNone);

        Self::new(from, to, captured, DignityNone, 0)
    }

    #[inline(always)]
    pub const fn quiet(from: Square, to: Square) -> Self {
        Self::new(from, to, DignityNone, DignityNone, 0)
    }

    #[inline(always)]
    pub const fn pawn_single(from: Square, to: Square) -> Self {
        always!(from.rank().index() >= Rank2.index());
        always!(from.rank().index() <= Rank7.index());

        Self::new(from, to, DignityNone, DignityNone, 0)
    }

    #[inline(always)]
    pub const fn pawn_double(from: Square, to: Square) -> Self {
        always!(from.rank() == Rank2 || from.rank() == Rank7);

        Self::new(from, to, DignityNone, DignityNone, 0)
    }

    #[inline(always)]
    pub const fn en_passant(from: Square, to: Square) -> Self {
        always!(
            (from.rank() == Rank4 && to.rank() == Rank3) ||
            (from.rank() == Rank5 && to.rank() == Rank6)
        );

        Self::new(from, to, Pawn, DignityNone, 1)
    }

    #[inline(always)]
    pub const fn promotion(from: Square, to: Square, promoted: Dignity) -> Self {
        always!(
            (from.rank() == Rank2 && to.rank() == Rank1) ||
            (from.rank() == Rank7 && to.rank() == Rank8)
        );
        always!(promoted != DignityNone);

        Self::new(from, to, DignityNone, promoted, 0)
    }

    #[inline(always)]
    pub const fn promotion_capture(
        from: Square,
        to: Square,
        captured: Dignity,
        promoted: Dignity,
    ) -> Self {
        always!(
            (from.rank() == Rank2 && to.rank() == Rank1) ||
            (from.rank() == Rank7 && to.rank() == Rank8)
        );
        always!(promoted != DignityNone);
        always!(captured != DignityNone);

        Self::new(from, to, captured, promoted, 0)
    }

    #[inline(always)]
    pub const fn king_side_castling(
        from: Square,
        to: Square,
    ) -> Self {
        always!(from.file() == FileE);
        always!(to.file() == FileG);

        Self::new(from, to, DignityNone, DignityNone, 0)
    }

    #[inline(always)]
    pub const fn queen_side_castling(
        from: Square,
        to: Square,
    ) -> Self {
        always!(from.file() == FileE);
        always!(to.file() == FileC);

        Self::new(from, to, DignityNone, DignityNone, 0)
    }

    #[inline(always)]
    pub const fn from(self) -> Square {
        let index = (self.0 as SquareInner) & Square::Mask;

        Square::from_index(index)
    }

    #[inline(always)]
    pub const fn to(self) -> Square {
        let index = ((self.0 >> 6) as SquareInner) & Square::Mask;

        Square::from_index(index)
    }

    #[inline(always)]
    pub const fn captured(self) -> Dignity {
        let index = ((self.0 >> 12) as DignityInner) & Dignity::Mask;

        Dignity::from_index(index)
    }

    #[inline(always)]
    pub const fn promoted(self) -> Dignity {
        let index = ((self.0 >> 15) as DignityInner) & Dignity::Mask;

        Dignity::from_index(index)
    }

    #[inline(always)]
    pub const fn index(self) -> MoveInner {
        self.0
    }

    #[inline(always)]
    pub const fn is_capture(self) -> bool {
        self.captured() != DignityNone
    }

    #[inline(always)]
    pub const fn is_pawn_double_move(self, moved: Dignity) -> bool {
        if moved != Pawn {
            return false;
        }

        ( // White move
            (self.from().rank() == Rank2) &&
            (self.to().rank() == Rank4)
        ) || ( // Black move
            (self.from().rank() == Rank7) &&
            (self.to().rank() == Rank5)
        )
    }

    #[inline(always)]
    pub const fn is_king_side_castling(self, moved: Dignity) -> bool {
        (self.from().file() == FileE) &&
        (self.to().file() == FileG) &&
        (moved == King)
    }

    #[inline(always)]
    pub const fn is_queen_side_castling(self, moved: Dignity) -> bool {
        (self.from().file() == FileE) &&
        (self.to().file() == FileC) &&
        (moved == King)
    }

    #[inline(always)]
    pub const fn is_en_passant(self) -> bool {
        self.index() & Self::EnPassantSpecial != 0
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ff, fr) = self.from().fen();
        let (tf, tr) = self.to().fen();
        let bytes = [ff, fr, tf, tr];
        let bytes = unsafe { std::str::from_utf8_unchecked(&bytes) };

        write!(f, "{bytes}")?;

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
