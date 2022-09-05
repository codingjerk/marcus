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

        let mut result: Self = unsafe { undefined() };

        let mut fen_index: usize = 0; // PERF: try smaller types

        macro_rules! fen_char {
            () => {
                unsafe {
                    always(fen_index < fen.len());
                    *fen.get_unchecked(fen_index)
                }
            }
        }

        // 1. Position
        let mut square = a8;
        loop {
            match fen_char!() {
                skip @ b'1' ..= b'8' => {
                    let skip = skip - b'0';

                    unsafe {
                        always(skip >= 1);
                        always(skip <= 8);
                    }

                    for _ in 0..skip {
                        result.set_piece_unchecked(square, PieceNone);
                        square.move_right_unchecked(1);
                    }
                },
                b'/' => square.move_down_unchecked(2),
                b' ' => {
                    unsafe { always(square == a2) }
                    fen_index += 1;
                    break;
                },
                piece => {
                    result.set_piece_unchecked(square, Piece::from_fen(piece));
                    square.move_right_unchecked(1);
                },
            }

            fen_index += 1;
        }

        // 2. Side to move
        result.side_to_move = match fen_char!() {
            b'b' => Black,
            b'w' => White,
            _ => unsafe { unreachable() },
        };
        fen_index += 1;

        // Skip space
        unsafe { always(fen[fen_index] == b' ') }
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
                    unsafe { always(fen[fen_index] == b' ') }
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
            unsafe { always(fen[fen_index] == b' ') }
            fen_index += 1;
        } else {
            let file = fen_char!();
            fen_index += 1;
            let rank = fen_char!();
            fen_index += 1;
            unsafe { always(fen[fen_index] == b' ') }
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
                // fen_index += 1;
                break;
            }

            let digit = fen_char!();
            unsafe { always(b'0' <= digit && digit <= b'9') }

            result.halfmove_clock *= 10;
            result.halfmove_clock += (digit - b'0') as HalfmoveClock;

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

    fn set_piece_unchecked(&mut self, at: Square, piece: Piece) {
        let index = at.index() as usize;
        unsafe { always(index < 64) }

        self.squares[index] = piece;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
