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

    // PERF: try to store only enpassant file
    // PERF: try to use "invalid enpassant square", like E4
    enpassant_square: Option<Square>,
}

impl Board {
    pub fn empty() -> Self {
        Board {
            squares: [PieceNone; 64],
            side_to_move: White,
            castling_rights: CastlingRightsNone,
            enpassant_square: None,
            halfmove_clock: 0,
        }
    }

    pub fn from_fen(fen: &[u8]) -> Self {
        unsafe {
            always(fen.len() >= MIN_FEN_SIZE);
            always(fen.len() <= MAX_FEN_SIZE);
        }

        // let mut result: Self = unsafe undefined() };
        let mut result = Self {
            squares: [PieceNone; 64],
            side_to_move: unsafe { undefined() },
            castling_rights: unsafe { undefined() },
            enpassant_square: unsafe { undefined() },
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

        macro_rules! expect_char {
            ($e:expr) => {
                let c = fen_char!();
                unsafe { always(c == $e) };
            }
        }

        // 1. Position
        let mut square = a8;
        let mut rank: u8 = 56;
        while rank <= 56 {
            let mut file: u8 = 0;
            while file < 8 {
                let c = fen_char!();
                if c <= b'8' {
                    file = file.wrapping_add(c.wrapping_sub(b'1'));
                } else {
                    let piece = Piece::from_fen(c);
                    let square = Square::from_index(file ^ rank);

                    result.set_piece_unchecked(square, piece);
                }

                file += 1;
                fen_index += 1;
            }

            rank = rank.wrapping_sub(8);
            fen_index += 1;
        }

        // 2. Side to move
        match fen_char!() {
            b'b' => { result.side_to_move = Black; },
            b'w' => { result.side_to_move = White; },
            _ => unsafe { unreachable() },
        }
        fen_index += 1;

        // Skip space
        expect_char!(b' ');
        fen_index += 1;

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
                    expect_char!(b' ');
                    fen_index += 1;
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
            result.enpassant_square = None;
            fen_index += 1;
            expect_char!(b' ');
            fen_index += 1;
        } else { // PERF: parse only file and calculate rank based on side_to_move
            let file = fen_char!();
            fen_index += 1;
            let rank = fen_char!();
            fen_index += 1;
            expect_char!(b' ');
            fen_index += 1;

            if result.side_to_move == White {
                unsafe { always(rank == b'6') }
            } else {
                unsafe { always(rank == b'3') }
            }

            result.enpassant_square = Some(Square::from_fen(file, rank));
        }

        // 5. Halfmove clock
        result.halfmove_clock = 0;
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
            result.halfmove_clock += (digit & 0b1111) as HalfmoveClock;

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

    pub const fn enpassant_square(&self) -> Option<Square> {
        self.enpassant_square
    }

    pub const fn halfmove_clock(&self) -> HalfmoveClock {
        self.halfmove_clock
    }

    // PERF: check if #[inline] works good here
    pub fn fen(&self, buffer: &mut StaticBuffer<u8, MAX_FEN_SIZE>) {
        // 1. Position
        let mut empty_count: u8 = 0;
        // TODO: use File, Rank and iterators
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Square::from_index(file ^ rank * 8);
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
            empty_count = 0;

            if rank != 0 {
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
        match self.enpassant_square() {
            Some(square) => {
                let (file, rank) = square.fen();
                buffer.add(file);
                buffer.add(rank);
            },
            None => buffer.add(b'-'),
        }
        buffer.add(b' ');

        // 5. Halfmove clock
        let hmc = self.halfmove_clock();
        unsafe { always(self.halfmove_clock <= 999) }

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

    fn set_piece_unchecked(&mut self, at: Square, piece: Piece) {
        let index = at.index() as usize;
        unsafe { always(index < 64) }

        self.squares[index] = piece;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn from_fen_enpassant() {
        for (fen, expected) in [
            (&b"8/8/8/8/8/8/8/8 b - - 0 1"[..], None),
            (&b"8/8/8/8/8/8/8/8 b - e3 0 1"[..], Some(e3)),
            (&b"8/8/8/8/8/8/8/8 w - c6 0 1"[..], Some(c6)),
        ] {
            let board = Board::from_fen(fen);
            assert_eq!(board.enpassant_square(), expected);
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
