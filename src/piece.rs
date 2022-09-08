use crate::prelude::*;

pub type DignityInner = u8; // PERF: try smaller and bigger types

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dignity(DignityInner);

impl Dignity {
    pub const Mask: DignityInner = 0b111;

    pub const fn from_index(index: DignityInner) -> Self {
        unsafe { always(index & Self::Mask == index) }

        Self(index)
    }

    pub const fn index(&self) -> DignityInner {
        self.0
    }
}

// PERF: try to numerate from 0
pub const DignityNone: Dignity = Dignity(0);

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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Piece(PieceInner);

impl Piece {
    pub const Mask: PieceInner = 0b1111;

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
        let index = (self.index() as DignityInner) & Dignity::Mask;

        Dignity::from_index(index)
    }

    pub const fn color(self) -> Color {
        let index = ((self.index() >> 3) as ColorInner) & Color::Mask;

        Color::from_index(index)
    }

    #[inline]
    #[allow(const_err)]
    pub const fn from_fen(fen: u8) -> Self {
        // Bit structure of input (fen char)
        // NOTE: works for ASCII encoding only
        // w or b are have this in common:
        //
        // 0 1 - - - - - -
        //     ^ \_______/ <- dignity (coded)
        //     | reversed color (1 is black, 0 is white)
        //
        // Coded dignity can be casted to real dignity
        // via inperfect hashing with lookup table
        const FEN_TO_PIECE: [Piece; 0b110011] = {
            let mut xs = [PieceNone; _];

            xs[0b110000] = BlackPawn;
            xs[0b101110] = BlackKnight;
            xs[0b100010] = BlackBishop;
            xs[0b110010] = BlackRook;
            xs[0b110001] = BlackQueen;
            xs[0b101011] = BlackKing;

            xs[0b010000] = WhitePawn;
            xs[0b001110] = WhiteKnight;
            xs[0b000010] = WhiteBishop;
            xs[0b010010] = WhiteRook;
            xs[0b010001] = WhiteQueen;
            xs[0b001011] = WhiteKing;

            xs
        };

        let hash = (fen & 0b111111) as usize;
        unsafe {
            always(hash <= 0b110010);
            *FEN_TO_PIECE.get_unchecked(hash)
        }
    }

    #[inline]
    #[allow(const_err)]
    pub const fn fen(self) -> u8 {
        const PIECE_TO_FEN: [u8; PieceMax.index() as usize + 1] = {
            let mut xs = [0; _];

            xs[BlackPawn.index() as usize] = b'p';
            xs[BlackKnight.index() as usize] = b'n';
            xs[BlackBishop.index() as usize] = b'b';
            xs[BlackRook.index() as usize] = b'r';
            xs[BlackQueen.index() as usize] = b'q';
            xs[BlackKing.index() as usize] = b'k';

            xs[WhitePawn.index() as usize] = b'P';
            xs[WhiteKnight.index() as usize] = b'N';
            xs[WhiteBishop.index() as usize] = b'B';
            xs[WhiteRook.index() as usize] = b'R';
            xs[WhiteQueen.index() as usize] = b'Q';
            xs[WhiteKing.index() as usize] = b'K';

            xs
        };

        let index = self.index();
        unsafe {
            always(index <= PieceMax.index());
            *PIECE_TO_FEN.get_unchecked(index as usize)
        }
    }

    pub fn rand(rng: &mut FastRng) -> Self {
        let dignity = Dignity::from_index(rng.rand_range_u8(1, 7));
        let color = Color::from_index(rng.rand_range_u8(0, 2));

        Self::new(color, dignity)
    }
}

pub const PieceNone: Piece = Piece::new(Color::from_index(0), DignityNone);

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

pub const PieceMax: Piece = WhiteKing;
