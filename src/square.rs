use crate::prelude::*;

pub type SquareInner = u8; // PERF: try smaller and bigger types

// Bit structure:
// - - - - - -
// \___/ \___/ <- file
//   | - rank
// Total bits: 3 + 3 = 6
#[derive(Copy, Clone, Debug, PartialEq)]
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

    pub const fn from_x_y(x: u8, y: u8) -> Self {
        Self::from_index(x ^ (y * 8))
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

    pub const fn x(self) -> u8 {
        self.0 % 8
    }

    pub const fn y(self) -> u8 {
        self.0 / 8
    }

    pub const fn file(self) -> File {
        File::from_index(self.x())
    }

    pub const fn rank(self) -> Rank {
        Rank::from_index(self.y() * 8)
    }

    #[inline]
    pub const fn fen(self) -> (u8, u8) {
        let file = b'a' + self.x();
        let rank = b'1' + self.y();

        unsafe {
            always(b'a' <= file && file <= b'h');
            always(b'1' <= rank && rank <= b'8');
        }

        (file, rank)
    }

    pub fn rand(rng: &mut FastRng) -> Self {
        Self::from_index(rng.rand_range_u8(0, 64))
    }

    pub const fn up(self, by: SquareInner) -> Self {
        Self::from_index(self.index() + by * 8)
    }

    pub const fn down(self, by: SquareInner) -> Self {
        Self::from_index(self.index() - by * 8)
    }

    pub const fn by(self, dx: i8, dy: i8) -> Option<Self> {
        let x = self.x() as i8 + dx;
        let y = self.y() as i8 + dy;

        if x < 0 || x >= 8 { return None }
        if y < 0 || y >= 8 { return None }

        Some(Self::from_x_y(x as u8, y as u8))
    }

    // Moves black pieces toward rank 1
    // And white pieces toward rank 8
    pub const fn forward(self, color: Color, by: SquareInner) -> Self {
        match color {
            Black => self.down(by),
            White => self.up(by),
            _ => unsafe { unreachable() },
        }
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

    pub const fn en_passant(
        side_to_move: Color,
        en_passant_file: File,
    ) -> Self {
        let en_passant_rank = Rank::en_passant(side_to_move);

        Self::from_file_rank(en_passant_file, en_passant_rank)
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

pub const a4: Square = Square(24);
pub const b4: Square = Square(25);
pub const c4: Square = Square(26);
pub const d4: Square = Square(27);
pub const e4: Square = Square(28);
pub const f4: Square = Square(29);
pub const g4: Square = Square(30);
pub const h4: Square = Square(31);

pub const a5: Square = Square(32);
pub const b5: Square = Square(33);
pub const c5: Square = Square(34);
pub const d5: Square = Square(35);
pub const e5: Square = Square(36);
pub const f5: Square = Square(37);
pub const g5: Square = Square(38);
pub const h5: Square = Square(39);

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

    pub const fn from_fen(fen: u8) -> Self {
        unsafe { always(b'a' <= fen && fen <= b'h') }

        Self::from_index(fen - b'a')
    }

    pub const fn a_to_h() -> FileIterator {
        FileIterator(FileA)
    }

    pub fn rand(rng: &mut FastRng) -> Self {
        Self::from_index(rng.rand_range_u8(0, 8))
    }

    // TODO: move en passant possibility to separate board field and
    // disallow to use more than 3 bits here
    pub const fn is_en_passant_none(self) -> bool {
        (self.0 & 64) != 0
    }

    pub const fn fen(self) -> u8 {
        b'a' + self.0
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

// NOTE: this is not valid File and even not valid Square
//       to catch if it will be used somethere except of checking
pub const FileEnPassantNone: File = File(64);

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
    pub const Mask: SquareInner = 0b111000;

    pub const fn from_index(index: SquareInner) -> Self {
        unsafe {
            always(index % 8 == 0);
            always(index <= 56);
        }

        Self(index)
    }

    pub const fn top_to_bottom() -> RevRankIterator {
        RevRankIterator(Rank8)
    }

    pub const fn pawn_double_rank(side_to_move: Color) -> Self {
        match side_to_move {
            Black => Rank7,
            White => Rank2,
            _ => unsafe { unreachable() },
        }
    }

    pub const fn en_passant(side_to_move: Color) -> Self {
        match side_to_move {
            Black => Rank3,
            White => Rank6,
            _ => unsafe { unreachable() },
        }
    }

    pub const fn fen(self) -> u8 {
        b'1' + (self.0 / 8)
    }
}

pub const Rank1: Rank = Rank::from_index(0);
pub const Rank2: Rank = Rank::from_index(8);
pub const Rank3: Rank = Rank::from_index(16);
pub const Rank4: Rank = Rank::from_index(24);
pub const Rank5: Rank = Rank::from_index(32);
pub const Rank6: Rank = Rank::from_index(40);
pub const Rank7: Rank = Rank::from_index(48);
pub const Rank8: Rank = Rank::from_index(56);

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
