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

// PERF: try numerate from 0
pub const Pawn: Dignity   = Dignity(1);
pub const Knight: Dignity = Dignity(2);
pub const Bishop: Dignity = Dignity(3);
pub const Rook: Dignity   = Dignity(4);
pub const Queen: Dignity  = Dignity(5);
pub const King: Dignity   = Dignity(6);

pub type PieceInner = u8; // PERF: try smaller and bigger types

// Bit structure:
// - - - -
// ^ \ _ / <- Dignity
// | - Color
// Total bits: 1 + 3 = 4
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece(PieceInner);

impl Piece {
    pub const Mask: PieceInner = 0b1111;

    pub const None: Self = Piece(0);

    pub const fn new(color: Color, dignity: Dignity) -> Self {
        let bits =
            (dignity.index() as PieceInner)
            ^ ((color.index() as PieceInner) << 3)
        ;

        unsafe { always(bits & Self::Mask == bits) }

        Self(bits)
    }

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

    pub fn from_fen(fen: u8) -> Self {
        match fen {
            b'p' => BlackPawn,
            b'n' => BlackKnight,
            b'b' => BlackBishop,
            b'r' => BlackRook,
            b'q' => BlackQueen,
            b'k' => BlackKing,

            b'P' => WhitePawn,
            b'N' => WhiteKnight,
            b'B' => WhiteBishop,
            b'R' => WhiteRook,
            b'Q' => WhiteQueen,
            b'K' => WhiteKing,

            _ => unsafe { unreachable() },
        }
    }
}

pub const BlackPawn: Piece = Piece::new(Black, Pawn);
pub const BlackKnight: Piece = Piece::new(Black, Knight);
pub const BlackBishop: Piece = Piece::new(Black, Bishop);
pub const BlackRook: Piece = Piece::new(Black, Rook);
pub const BlackQueen: Piece = Piece::new(Black, Queen);
pub const BlackKing: Piece = Piece::new(Black, King);

pub const WhitePawn: Piece = Piece::new(White, Pawn);
pub const WhiteKnight: Piece = Piece::new(White, Knight);
pub const WhiteBishop: Piece = Piece::new(White, Bishop);
pub const WhiteRook: Piece = Piece::new(White, Rook);
pub const WhiteQueen: Piece = Piece::new(White, Queen);
pub const WhiteKing: Piece = Piece::new(White, King);
