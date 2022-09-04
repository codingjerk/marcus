use crate::prelude::*;

type Inner = u32; // PERF: try smaller and bigger types

// Bit structure:
// - - - - - - - - - - - - -
//   ^   [  from ] [  to   ]
//   | - captured piece
// Total bits: 5 + 5 + 3 = 13
pub struct Move(Inner);

impl Move {
    // PERF: try to store Piece instead of Dignity
    pub const fn new(from: Square, to: Square, captured: Dignity) -> Self {
        let bits =
            (from.index() as Inner)
            ^ ((to.index() as Inner) << 6)
            ^ ((captured.index() as Inner) << 12)
        ;

        unsafe { always(true) }

        Self(bits)
    }

    pub const fn capture(from: Square, to: Square, captured: Dignity) -> Self {
        Self::new(from, to, captured)
    }

    pub const fn quiet(from: Square, to: Square) -> Self {
        Self::new(from, to, Dignity::None)
    }

    pub const fn index(self) -> Inner {
        self.0
    }
}
