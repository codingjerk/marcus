use rand::Rng;

use crate::prelude::*;

pub type SquareInner = u8; // PERF: try smaller and bigger types

// Bit structure:
// - - - - - -
// \___/ \___/ <- file
//   | - rank
// Total bits: 3 + 3 = 6
#[derive(Copy, Clone, Debug, PartialEq)]
// TODO: make inner value private
pub struct Square(SquareInner);

impl Square {
    pub const Mask: SquareInner = 0b111111;

    pub const fn from_index(index: SquareInner) -> Self {
        unsafe { always(index & Self::Mask == index) }

        Self(index)
    }

    pub const fn from_file_rank(file: File, rank: Rank) -> Self {
        Self::from_index(file.0 ^ rank.0)
    }

    pub const fn from_fen(file: u8, rank: u8) -> Self {
        unsafe {
            always(b'a' <= file && file <= b'h');
            always(b'1' <= rank && rank <= b'8');
        }

        let index = (file - b'a') + (rank - b'1') * 8;

        Self::from_index(index)
    }

    pub const fn index(self) -> SquareInner {
        self.0
    }

    pub const fn iter() -> SquareIterator {
        SquareIterator(a1)
    }

    #[inline]
    pub const fn fen(self) -> (u8, u8) {
        let file = b'a' + (self.0 % 8);
        let rank = b'1' + (self.0 / 8);

        unsafe {
            always(b'a' <= file && file <= b'h');
            always(b'1' <= rank && rank <= b'8');
        }

        (file, rank)
    }

    pub fn rand<R: Rng>(rng: &mut R) -> Self {
        Self::from_index(rng.gen_range(0..64))
    }

    pub fn move_right_unchecked(&mut self, by: SquareInner) {
        unsafe {
            always(self.0 + by <= 100);

            self.0 = self.0.unchecked_add(by);
        }
    }

    pub fn move_down_unchecked(&mut self, by: SquareInner) {
        self.0 = self.0.wrapping_sub(by * 8);
    }
}

pub struct SquareIterator(Square);

impl Iterator for SquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.0).0 > h8.0 {
            return None
        }

        let result = Some(self.0);
        self.0.move_right_unchecked(1);

        result
    }
}

pub const a1: Square = Square(0);
pub const b1: Square = Square(1);
pub const c1: Square = Square(2);
pub const d1: Square = Square(3);
pub const e1: Square = Square(4);
pub const f1: Square = Square(5);
pub const g1: Square = Square(6);
pub const h1: Square = Square(7);

pub const a2: Square = Square(8);
pub const b2: Square = Square(9);
pub const c2: Square = Square(10);
pub const d2: Square = Square(11);
pub const e2: Square = Square(12);
pub const f2: Square = Square(13);
pub const g2: Square = Square(14);
pub const h2: Square = Square(15);

pub const a3: Square = Square(16);
pub const b3: Square = Square(17);
pub const c3: Square = Square(18);
pub const d3: Square = Square(19);
pub const e3: Square = Square(20);
pub const f3: Square = Square(21);
pub const g3: Square = Square(22);
pub const h3: Square = Square(23);

pub const a6: Square = Square(40);
pub const b6: Square = Square(41);
pub const c6: Square = Square(42);
pub const d6: Square = Square(43);
pub const e6: Square = Square(44);
pub const f6: Square = Square(45);
pub const g6: Square = Square(46);
pub const h6: Square = Square(47);

pub const a7: Square = Square(48);
pub const b7: Square = Square(49);
pub const c7: Square = Square(50);
pub const d7: Square = Square(51);
pub const e7: Square = Square(52);
pub const f7: Square = Square(53);
pub const g7: Square = Square(54);
pub const h7: Square = Square(55);

pub const a8: Square = Square(56);
pub const b8: Square = Square(57);
pub const c8: Square = Square(58);
pub const d8: Square = Square(59);
pub const e8: Square = Square(60);
pub const f8: Square = Square(61);
pub const g8: Square = Square(62);
pub const h8: Square = Square(63);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct File(SquareInner);

impl File {
    pub const Mask: SquareInner = 0b000111;

    pub const fn from_index(index: SquareInner) -> Self {
        unsafe { always(index & Self::Mask == index) }

        Self(index)
    }

    pub const fn a_to_h() -> FileIterator {
        FileIterator(FileA)
    }

    pub fn rand<R: Rng>(rng: &mut R) -> Self {
        Self::from_index(rng.gen_range(0..8))
    }
}

pub const FileA: File = File(0);
pub const FileB: File = File(1);
pub const FileC: File = File(2);
pub const FileD: File = File(3);
pub const FileE: File = File(4);
pub const FileF: File = File(5);
pub const FileG: File = File(6);
pub const FileH: File = File(7);

pub struct FileIterator(File);

impl Iterator for FileIterator {
    type Item = File;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.0).0 > FileH.0 {
            return None
        }

        let result = Some(self.0);
        (self.0).0 += 1;

        result
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rank(SquareInner);

impl Rank {
    pub const Mask: SquareInner = 0b000111;

    pub const fn top_to_bottom() -> RevRankIterator {
        RevRankIterator(Rank8)
    }

    pub const fn en_passant(color: Color) -> Self {
        match color {
            Black => Rank3,
            White => Rank6,
            _ => unsafe { unreachable() },
        }
    }
}

pub const Rank1: Rank = Rank(0);
pub const Rank2: Rank = Rank(8);
pub const Rank3: Rank = Rank(16);
pub const Rank4: Rank = Rank(24);
pub const Rank5: Rank = Rank(32);
pub const Rank6: Rank = Rank(40);
pub const Rank7: Rank = Rank(48);
pub const Rank8: Rank = Rank(56);

pub struct RevRankIterator(Rank);

impl Iterator for RevRankIterator {
    type Item = Rank;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.0).0 > Rank8.0 {
            return None
        }

        let result = Some(self.0);
        (self.0).0 = (self.0).0.wrapping_sub(8);

        result
    }
}
