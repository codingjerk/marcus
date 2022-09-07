use crate::prelude::*;

// TODO: calculate
const MAX_MOVE_BUFFER_SIZE: usize = 100;

type MoveBuffer = StaticBuffer<Move, MAX_MOVE_BUFFER_SIZE>;

// TODO: move to mailbox8x8
pub struct MoveGenerator;

impl MoveGenerator {
    pub const fn new() -> Self {
        Self
    }

    pub fn generate(
        &self,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        for square in Square::iter() {
            let piece = board.piece(square);
            if piece.color() != board.side_to_move() {
                continue;
            }

            let piece_gen = match piece.dignity() {
                p if p == Pawn => Self::generate_for_pawn,
                p if p == Knight => Self::generate_for_knight,
                p if p == Bishop => Self::generate_for_bishop,
                p if p == Rook => Self::generate_for_rook,
                p if p == Queen => Self::generate_for_queen,
                _ => continue,
            };

            piece_gen(self, square, board, buffer);
        }
    }

    fn generate_for_pawn(
        &self,
        from: Square,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        let c = board.side_to_move();
        buffer.add(Move::pawn_single(from, from.forward(c, 1)));
        buffer.add(Move::pawn_double(from, from.forward(c, 2)));
    }

    fn generate_for_knight(
        &self,
        from: Square,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        for (dx, dy) in [
            (-2, -1),
            (-1, -2),
            (-2,  1),
            (-1,  2),
            ( 2, -1),
            ( 1, -2),
            ( 2,  1),
            ( 1,  2),
        ] {
            let to = match from.by(dx, dy) {
                Some(s) => s,
                None => continue,
            };

            if board.piece(to) == PieceNone {
                buffer.add(Move::quiet(from, to));
            }
        }
    }

    fn generate_for_bishop(
        &self,
        from: Square,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        for (dx, dy) in [
            (-1, -1),
            (-1,  1),
            ( 1, -1),
            ( 1,  1),
        ] {
            for d in 1..8 {
                let to = match from.by(dx * d, dy * d) {
                    Some(s) => s,
                    None => break,
                };

                let dest = board.piece(to);
                if dest == PieceNone {
                    buffer.add(Move::quiet(from, to));
                } else {
                    if dest.color() != board.side_to_move() {
                        buffer.add(Move::capture(from, to, dest.dignity()));
                    }
                    break;
                }
            }
        }
    }

    fn generate_for_rook(
        &self,
        from: Square,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        for (dx, dy) in [
            ( 0,  1),
            ( 0, -1),
            ( 1,  0),
            (-1,  0),
        ] {
            for d in 1..8 {
                let to = match from.by(dx * d, dy * d) {
                    Some(s) => s,
                    None => break,
                };

                let dest = board.piece(to);
                if dest == PieceNone {
                    buffer.add(Move::quiet(from, to));
                } else {
                    if dest.color() != board.side_to_move() {
                        buffer.add(Move::capture(from, to, dest.dignity()));
                    }
                    break;
                }
            }
        }
    }

    fn generate_for_queen(
        &self,
        from: Square,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        for (dx, dy) in [
            // Bishop directions
            (-1, -1),
            (-1,  1),
            ( 1, -1),
            ( 1,  1),

            // Rook directions
            ( 0,  1),
            ( 0, -1),
            ( 1,  0),
            (-1,  0),
        ] {
            for d in 1..8 {
                let to = match from.by(dx * d, dy * d) {
                    Some(s) => s,
                    None => break,
                };

                let dest = board.piece(to);
                if dest == PieceNone {
                    buffer.add(Move::quiet(from, to));
                } else {
                    if dest.color() != board.side_to_move() {
                        buffer.add(Move::capture(from, to, dest.dignity()));
                    }
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startpos() {
        let board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let movegen = MoveGenerator::new();
        let mut buffer = MoveBuffer::new();

        movegen.generate(&board, &mut buffer);

        assert!(buffer.contains(Move::pawn_double(a2, a4)));
        assert!(buffer.contains(Move::pawn_double(b2, b4)));
        assert!(buffer.contains(Move::pawn_double(c2, c4)));
        assert!(buffer.contains(Move::pawn_double(d2, d4)));
        assert!(buffer.contains(Move::pawn_double(e2, e4)));
        assert!(buffer.contains(Move::pawn_double(f2, f4)));
        assert!(buffer.contains(Move::pawn_double(g2, g4)));
        assert!(buffer.contains(Move::pawn_double(h2, h4)));

        assert!(buffer.contains(Move::pawn_single(a2, a3)));
        assert!(buffer.contains(Move::pawn_single(b2, b3)));
        assert!(buffer.contains(Move::pawn_single(c2, c3)));
        assert!(buffer.contains(Move::pawn_single(d2, d3)));
        assert!(buffer.contains(Move::pawn_single(e2, e3)));
        assert!(buffer.contains(Move::pawn_single(f2, f3)));
        assert!(buffer.contains(Move::pawn_single(g2, g3)));
        assert!(buffer.contains(Move::pawn_single(h2, h3)));

        assert!(buffer.contains(Move::quiet(b1, a3)));
        assert!(buffer.contains(Move::quiet(b1, c3)));
        assert!(buffer.contains(Move::quiet(g1, f3)));
        assert!(buffer.contains(Move::quiet(g1, h3)));

        assert_eq!(buffer.len(), 20);
    }

    #[test]
    fn bishop() {
        let board = Board::from_fen(b"8/8/5r2/8/3B4/8/5B2/8 w - - 0 1");
        let movegen = MoveGenerator::new();
        let mut buffer = MoveBuffer::new();

        movegen.generate(&board, &mut buffer);

        // d4 bishop
        assert!(buffer.contains(Move::quiet(d4, c5)));
        assert!(buffer.contains(Move::quiet(d4, b6)));
        assert!(buffer.contains(Move::quiet(d4, a7)));

        assert!(buffer.contains(Move::quiet(d4, c3)));
        assert!(buffer.contains(Move::quiet(d4, b2)));
        assert!(buffer.contains(Move::quiet(d4, a1)));

        assert!(buffer.contains(Move::quiet(d4, e5)));
        assert!(buffer.contains(Move::capture(d4, f6, Rook)));

        assert!(buffer.contains(Move::quiet(d4, e3)));

        // f2 bishop
        assert!(buffer.contains(Move::quiet(f2, e3)));
        assert!(buffer.contains(Move::quiet(f2, e1)));
        assert!(buffer.contains(Move::quiet(f2, g1)));
        assert!(buffer.contains(Move::quiet(f2, g3)));
        assert!(buffer.contains(Move::quiet(f2, h4)));

        assert_eq!(buffer.len(), 14);
    }

    #[test]
    fn rook() {
        let board = Board::from_fen(b"8/8/8/1n6/8/8/1R4R1/8 w - - 0 1");
        let movegen = MoveGenerator::new();
        let mut buffer = MoveBuffer::new();

        movegen.generate(&board, &mut buffer);

        // b2 rook
        assert!(buffer.contains(Move::quiet(b2, b1)));
        assert!(buffer.contains(Move::quiet(b2, b3)));
        assert!(buffer.contains(Move::quiet(b2, b4)));
        assert!(buffer.contains(Move::capture(b2, b5, Knight)));

        assert!(buffer.contains(Move::quiet(b2, a2)));
        assert!(buffer.contains(Move::quiet(b2, c2)));
        assert!(buffer.contains(Move::quiet(b2, d2)));
        assert!(buffer.contains(Move::quiet(b2, e2)));
        assert!(buffer.contains(Move::quiet(b2, f2)));

        // g2 rook
        assert!(buffer.contains(Move::quiet(g2, c2)));
        assert!(buffer.contains(Move::quiet(g2, d2)));
        assert!(buffer.contains(Move::quiet(g2, e2)));
        assert!(buffer.contains(Move::quiet(g2, f2)));
        assert!(buffer.contains(Move::quiet(g2, h2)));

        assert!(buffer.contains(Move::quiet(g2, g1)));
        assert!(buffer.contains(Move::quiet(g2, g3)));
        assert!(buffer.contains(Move::quiet(g2, g4)));
        assert!(buffer.contains(Move::quiet(g2, g5)));
        assert!(buffer.contains(Move::quiet(g2, g6)));
        assert!(buffer.contains(Move::quiet(g2, g7)));
        assert!(buffer.contains(Move::quiet(g2, g8)));

        assert_eq!(buffer.len(), 21);
    }

    #[test]
    fn queen() {
        let board = Board::from_fen(b"8/n7/3p1b2/8/1k1Q1r2/2q1n3/3p4/8 w - - 0 1");
        let movegen = MoveGenerator::new();
        let mut buffer = MoveBuffer::new();

        movegen.generate(&board, &mut buffer);

        assert!(buffer.contains(Move::quiet(d4, d5)));
        assert!(buffer.contains(Move::capture(d4, d6, Pawn)));

        assert!(buffer.contains(Move::quiet(d4, e5)));
        assert!(buffer.contains(Move::capture(d4, f6, Bishop)));

        assert!(buffer.contains(Move::quiet(d4, e4)));
        assert!(buffer.contains(Move::capture(d4, f4, Rook)));

        assert!(buffer.contains(Move::capture(d4, e3, Knight)));

        assert!(buffer.contains(Move::quiet(d4, d3)));
        assert!(buffer.contains(Move::capture(d4, d2, Pawn)));

        assert!(buffer.contains(Move::capture(d4, c3, Queen)));

        assert!(buffer.contains(Move::quiet(d4, c4)));
        assert!(buffer.contains(Move::capture(d4, b4, King)));

        assert!(buffer.contains(Move::quiet(d4, c5)));
        assert!(buffer.contains(Move::quiet(d4, b6)));
        assert!(buffer.contains(Move::capture(d4, a7, Knight)));

        assert_eq!(buffer.len(), 15);
    }
}
