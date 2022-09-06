use rand::Rng;

use crate::prelude::*;

type HalfmoveClock = u16; // PERF: try smaller and bigger types
const MAX_HALFMOVE_CLOCK: HalfmoveClock = 999;
const MIN_FEN_SIZE: usize = 24;
const MAX_FEN_SIZE: usize = 90;

// TODO: move to board/mailbox8x8
#[derive(Debug, PartialEq)]
pub struct Board {
    squares: [Piece; 64],

    // PERF: try to merge flags
    side_to_move: Color,
    halfmove_clock: HalfmoveClock,

    // Undo list
    // TODO: move to array
    // PERF: try different memory layouts
    castling_rights: CastlingRights,
    en_passant_file: File,
}

impl Board {
    pub fn empty() -> Self {
        Board {
            squares: [PieceNone; 64],
            side_to_move: White,
            castling_rights: CastlingRightsNone,
            en_passant_file: FileEnPassantNone,
            halfmove_clock: 0,
        }
    }

    // TODO: use something like read buffer to share cursor (fen_index)
    pub fn from_fen(fen: &[u8]) -> Self {
        unsafe {
            always(fen.len() >= MIN_FEN_SIZE);
            always(fen.len() <= MAX_FEN_SIZE);
        }

        let mut result = Self {
            squares: [PieceNone; 64],
            side_to_move: unsafe { undefined() },
            castling_rights: unsafe { undefined() },
            en_passant_file: unsafe { undefined() },
            halfmove_clock: unsafe { undefined() },
        };

        let mut fen_index: u8 = 0;

        macro_rules! fen_char {
            () => {
                unsafe {
                    always((fen_index as usize) < fen.len());
                    *fen.get_unchecked(fen_index as usize)
                }
            }
        }

        macro_rules! skip_char {
            ($e:expr) => {
                let c = fen_char!();
                unsafe { always(c == $e) };
                fen_index += 1;
            }
        }

        // 1. Position
        // NOTE: following code depends on specific square representation
        //       so there are asserts for that
        unsafe {
            always(a1.index() == 0);
            always(a8.index() == 56);
            always(h8.index() == 63);
        }

        for rank in Rank::top_to_bottom() {
            let mut file: u8 = 0;
            while file < 8 {
                let c = fen_char!();

                // HACK: here we're doing dirty hack to just ignore
                //       any values lesser than 1 (/ and space in valid fen)
                //       to just keep single if here
                if c <= b'8' {
                    file = file.wrapping_add(c.wrapping_sub(b'1'));
                } else {
                    let piece = Piece::from_fen(c);
                    let square = Square::from_file_rank(File::from_index(file), rank);

                    result.set_piece_unchecked(square, piece);
                }

                file += 1;
                fen_index += 1;
            }

            fen_index += 1;
        }

        // 2. Side to move
        result.side_to_move = Color::from_fen(fen_char!());
        fen_index += 1;

        // Skip space
        skip_char!(b' ');

        // 3. Castling rights
        result.castling_rights.unset();
        loop {
            match fen_char!() {
                b' ' => {
                    fen_index += 1;
                    break;
                },
                b'-' => {
                    fen_index += 1;
                    skip_char!(b' ');
                    break;
                },
                right => {
                    result.castling_rights.set_from_fen(right);
                },
            }

            fen_index += 1;
        }

        // 4. En passant target square
        if fen_char!() == b'-' {
            result.en_passant_file = FileEnPassantNone;
            fen_index += 1;
            skip_char!(b' ');
        } else {
            result.en_passant_file = File::from_fen(fen_char!());
            fen_index += 1;

            if result.side_to_move == White {
                skip_char!(b'6');
            } else {
                skip_char!(b'3');
            }

            skip_char!(b' ');
        }

        // 5. Halfmove clock
        result.halfmove_clock = 0;
        // PERF: unroll loop
        // PERF: use array with powers of 10
        loop {
            if fen_char!() == b' ' {
                // NOTE: we don't move forward (fen_index += 1),
                //       cause we don't parse fullmove counter
                //       at all
                break;
            }

            let digit = fen_char!();
            unsafe { always(b'0' <= digit && digit <= b'9') }

            result.halfmove_clock *= 10;
            // NOTE: x & 0b1111 is equivalent to x - b'0' (if b'0' <= digit <= b'9')
            //       x ^ 0b110000 is the same thing for that range
            result.halfmove_clock += (digit ^ 0b110000) as HalfmoveClock;

            fen_index += 1;
        }

        // 6. Fullmove counter
        // NOTE: fullmove counter is ignored

        result
    }

    pub const fn piece(&self, at: Square) -> Piece {
        let index = at.index() as usize;
        unsafe { always(index < 64) }

        self.squares[index]
    }

    pub const fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub const fn castling_rights(&self) -> CastlingRights {
        self.castling_rights
    }

    pub const fn en_passant_file(&self) -> File {
        self.en_passant_file
    }

    pub const fn halfmove_clock(&self) -> HalfmoveClock {
        self.halfmove_clock
    }

    // PERF: check if #[inline] works good here
    pub fn fen(&self, buffer: &mut StaticBuffer<u8, MAX_FEN_SIZE>) {
        // 1. Position
        for rank in Rank::top_to_bottom() {
            let mut empty_count: u8 = 0;
            for file in File::a_to_h() {
                let square = Square::from_file_rank(file, rank);
                let piece = self.piece(square);

                if piece == PieceNone {
                    empty_count += 1;
                } else {
                    if empty_count != 0 {
                        buffer.add(b'0' ^ empty_count);
                    }
                    empty_count = 0;
                    buffer.add(piece.fen());
                }
            }

            if empty_count != 0 {
                buffer.add(b'0' ^ empty_count);
            }

            if rank != Rank1 {
                buffer.add(b'/');
            }
        }

        buffer.add(b' ');

        // 2. Side to move
        buffer.add(self.side_to_move().fen());
        buffer.add(b' ');

        // 3. Castling rights
        self.castling_rights().fen(buffer);
        buffer.add(b' ');

        // 4. En passant target square
        let ep = self.en_passant_file();
        if ep.is_en_passant_none() {
            buffer.add(b'-');
        } else {
            let rank = Rank::en_passant(self.side_to_move());
            buffer.add(ep.fen());
            buffer.add(rank.fen());
        }
        buffer.add(b' ');

        // 5. Halfmove clock
        let hmc = self.halfmove_clock();
        unsafe { always(self.halfmove_clock <= MAX_HALFMOVE_CLOCK) }

        let x = (hmc / 100 % 10) as u8;
        let y = (hmc / 10 % 10) as u8;
        let z = (hmc % 10) as u8;

        if x != 0 {
            // NOTE: b'0' ^ x is equivalent of b'0' + x (if 0 <= x <= 9)
            buffer.add(b'0' ^ x);
            buffer.add(b'0' ^ y);
            buffer.add(b'0' ^ z);
        } else if y != 0 {
            buffer.add(b'0' ^ y);
            buffer.add(b'0' ^ z);
        } else {
            buffer.add(b'0' ^ z);
        }

        buffer.add(b' ');

        // 6. Fullmove counter
        // NOTE: isn't used
        buffer.add(b'1');
    }

    // Creates random board, using `rng`
    // NOTE: this board can be invalid chess board
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut result = Self::empty();

        // 1. Position
        for square in Square::iter() {
            if rng.gen() {
                result.set_piece_unchecked(square, Piece::rand(rng));
            }
        }

        // 2. Side to move
        if rng.gen() { result.side_to_move = Black }

        // 3. Castling rights
        if rng.gen() { result.castling_rights.allow(BlackKingSide) }
        if rng.gen() { result.castling_rights.allow(BlackQueenSide) }
        if rng.gen() { result.castling_rights.allow(WhiteKingSide) }
        if rng.gen() { result.castling_rights.allow(WhiteQueenSide) }

        // 4. En passant target square
        if rng.gen() {
            result.en_passant_file = File::rand(rng);
        }

        // 5. Halfmove clock
        result.halfmove_clock = rng.gen_range(0..MAX_HALFMOVE_CLOCK);

        // 6. Fullmove counter
        // NOTE: isn't needed

        result
    }

    fn set_piece_unchecked(&mut self, at: Square, piece: Piece) {
        let index = at.index() as usize;
        unsafe { always(index < 64) }

        self.squares[index] = piece;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    type FenBuffer = StaticBuffer::<u8, MAX_FEN_SIZE>;

    #[test]
    fn from_fen_empty() {
        let board = Board::from_fen(b"8/8/8/8/8/8/8/8 w - - 0 1");
        assert_eq!(board, Board::empty());
        assert_eq!(board.piece(a1), PieceNone);
    }

    #[test]
    fn from_fen_startpos() {
        let board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert_eq!(board.piece(a1), WhiteRook);
        assert_eq!(board.piece(b1), WhiteKnight);
        assert_eq!(board.piece(c1), WhiteBishop);
        assert_eq!(board.piece(d1), WhiteQueen);
        assert_eq!(board.piece(e1), WhiteKing);
        assert_eq!(board.piece(f1), WhiteBishop);
        assert_eq!(board.piece(g1), WhiteKnight);
        assert_eq!(board.piece(h1), WhiteRook);

        assert_eq!(board.piece(a2), WhitePawn);
        assert_eq!(board.piece(b2), WhitePawn);
        assert_eq!(board.piece(c2), WhitePawn);
        assert_eq!(board.piece(d2), WhitePawn);
        assert_eq!(board.piece(e2), WhitePawn);
        assert_eq!(board.piece(f2), WhitePawn);
        assert_eq!(board.piece(g2), WhitePawn);
        assert_eq!(board.piece(h2), WhitePawn);

        assert_eq!(board.piece(a7), BlackPawn);
        assert_eq!(board.piece(b7), BlackPawn);
        assert_eq!(board.piece(c7), BlackPawn);
        assert_eq!(board.piece(d7), BlackPawn);
        assert_eq!(board.piece(e7), BlackPawn);
        assert_eq!(board.piece(f7), BlackPawn);
        assert_eq!(board.piece(g7), BlackPawn);
        assert_eq!(board.piece(h7), BlackPawn);

        assert_eq!(board.piece(a8), BlackRook);
        assert_eq!(board.piece(b8), BlackKnight);
        assert_eq!(board.piece(c8), BlackBishop);
        assert_eq!(board.piece(d8), BlackQueen);
        assert_eq!(board.piece(e8), BlackKing);
        assert_eq!(board.piece(f8), BlackBishop);
        assert_eq!(board.piece(g8), BlackKnight);
        assert_eq!(board.piece(h8), BlackRook);
    }

    #[test]
    fn from_fen_side_to_move() {
        for (fen, expected) in [
            (b"8/8/8/8/8/8/8/8 w - - 0 1", White),
            (b"8/8/8/8/8/8/8/8 b - - 0 1", Black),
        ] {
            let board = Board::from_fen(fen);
            assert_eq!(board.side_to_move(), expected);
        }
    }

    #[test]
    fn from_fen_castling() {
        for (fen, expected) in [
            (&b"8/8/8/8/8/8/8/8 w - - 0 1"[..], CastlingRightsNone),
            (&b"8/8/8/8/8/8/8/8 w KQkq - 0 1"[..], CastlingRightsAll),
            (&b"8/8/8/8/8/8/8/8 w K - 0 1"[..], WhiteKingSide),
            (&b"8/8/8/8/8/8/8/8 w k - 0 1"[..], BlackKingSide),
        ] {
            let board = Board::from_fen(fen);
            assert_eq!(board.castling_rights(), expected);
        }
    }

    #[test]
    fn from_fen_en_passant() {
        for (fen, expected) in [
            (&b"8/8/8/8/8/8/8/8 b - - 0 1"[..], FileEnPassantNone),
            (&b"8/8/8/8/8/8/8/8 b - e3 0 1"[..], FileE),
            (&b"8/8/8/8/8/8/8/8 w - c6 0 1"[..], FileC),
        ] {
            let board = Board::from_fen(fen);
            assert_eq!(board.en_passant_file(), expected);
        }
    }

    #[test]
    fn from_fen_halfmove_clock() {
        for (fen, expected) in [
            (&b"8/8/8/8/8/8/8/8 w - - 0 1"[..], 0),
            (&b"8/8/8/8/8/8/8/8 w - - 123 1"[..], 123),
            (&b"8/8/8/8/8/8/8/8 w - - 999 1"[..], 999),
            (&b"8/8/8/8/8/8/8/8 w - - 100 1"[..], 100),
        ] {
            let board = Board::from_fen(fen);
            assert_eq!(board.halfmove_clock(), expected);
        }
    }

    #[test]
    fn to_fen_startpos() {
        let fen = b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen(fen);

        let mut buffer = FenBuffer::new();
        board.fen(&mut buffer);
        assert_eq!(buffer.as_slice(), fen);
    }

    #[test]
    fn to_fen_trailing_empty_count() {
        let fen = b"r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
        let board = Board::from_fen(fen);

        let mut buffer = FenBuffer::new();
        board.fen(&mut buffer);
        assert_eq!(buffer.as_slice(), fen);
    }

    #[test]
    fn to_fen_castling() {
        let examples: [&[u8]; _] = [
            b"r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            b"r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
            b"r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 1",
        ];

        let mut buffer = FenBuffer::new();
        for fen in examples {
            let board = Board::from_fen(fen);
            buffer.reset();
            board.fen(&mut buffer);

            assert_eq!(buffer.as_slice(), fen);
        }
    }

    #[test]
    fn to_fen_en_passant() {
        let examples: [&[u8]; _] = [
            b"rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            b"rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 1",
        ];

        let mut buffer = FenBuffer::new();
        for fen in examples {
            let board = Board::from_fen(fen);
            buffer.reset();
            board.fen(&mut buffer);

            assert_eq!(buffer.as_slice(), fen);
        }
    }

    #[test]
    fn to_fen_hmc_fmc() {
        let examples: [&[u8]; _] = [
            b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 9 1",
            b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 56 1",
            b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 20 1",
            b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 99 1",
            b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 999 1",
        ];

        let mut buffer = FenBuffer::new();
        for fen in examples {
            let board = Board::from_fen(fen);
            buffer.reset();
            board.fen(&mut buffer);

            assert_eq!(buffer.as_slice(), fen);
        }
    }

    #[test]
    fn rand() {
        let mut rng = XorShiftRng::from_entropy();

        let mut buffer1 = FenBuffer::new();
        let board1 = Board::rand(&mut rng);
        board1.fen(&mut buffer1);

        let mut buffer2 = FenBuffer::new();
        let board2 = Board::rand(&mut rng);
        board2.fen(&mut buffer2);

        // NOTE: theoreticaly it is possible to randomly
        //       get two identical positions,
        //       but it should be **really** rare
        assert_ne!(buffer1.as_slice(), buffer2.as_slice());
    }

    // NOTE: This test is really slow and should be run rarely
    #[test]
    #[ignore]
    fn fuzz_fen() {
        // PERF: use own RNG
        let mut rng = XorShiftRng::from_entropy();
        let mut buffer = FenBuffer::new();
        let mut next_buffer = FenBuffer::new();

        for _ in 0..(72_993 * FUZZ_MULTIPLIER) {
            let board = Board::rand(&mut rng);
            buffer.reset();
            board.fen(&mut buffer);
            let fen = buffer.as_slice();

            let next_board = Board::from_fen(fen);
            next_buffer.reset();
            next_board.fen(&mut next_buffer);
            let next_fen = next_buffer.as_slice();

            assert_eq!(fen, next_fen);
        }
    }
}

#[cfg(test)]
mod bench {
    use super::*;

    use test::{Bencher, black_box};

    #[bench]
    fn from_fen_startpos(b: &mut Bencher) {
        b.iter(|| {
            let fen = black_box(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            Board::from_fen(fen)
        })
    }

    #[bench]
    fn from_fen_fullpos(b: &mut Bencher) {
        b.iter(|| {
            let fen = black_box(b"rnbqkbnr/pppppppp/qqqqqqqq/qqqqqqqq/QQQQQQQQ/QQQQQQQQ/PPPPPPPP/RNBQKBNR w KQkq e6 0 1");
            Board::from_fen(fen)
        })
    }

    #[bench]
    fn to_fen_empty(b: &mut Bencher) {
        let board = black_box(Board::empty());
        let mut buffer = StaticBuffer::<u8, MAX_FEN_SIZE>::new();

        b.iter(|| {
            buffer.reset();
            board.fen(&mut buffer);
        })
    }

    #[bench]
    fn to_fen_startpos(b: &mut Bencher) {
        let board = black_box(Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
        let mut buffer = StaticBuffer::<u8, MAX_FEN_SIZE>::new();

        b.iter(|| {
            buffer.reset();
            board.fen(&mut buffer);
        })
    }

    #[bench]
    fn to_fen_fullpos(b: &mut Bencher) {
        let board = black_box(Board::from_fen(b"rnbqkbnr/pppppppp/qqqqqqqq/qqqqqqqq/QQQQQQQQ/QQQQQQQQ/PPPPPPPP/RNBQKBNR w KQkq e6 0 1"));
        let mut buffer = StaticBuffer::<u8, MAX_FEN_SIZE>::new();

        b.iter(|| {
            buffer.reset();
            board.fen(&mut buffer);
        })
    }
}
