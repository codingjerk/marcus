use crate::prelude::*;

// TODO: calculate
const MAX_MOVE_BUFFER_SIZE: usize = 100;

// TODO: move to mailbox8x8
pub struct MoveGenerator;

impl MoveGenerator {
    pub const fn new() -> Self {
        Self
    }

    pub fn generate(
        &self,
        board: &Board,
        buffer: &mut StaticBuffer<Move, MAX_MOVE_BUFFER_SIZE>,
    ) {
        for square in Square::iter() {
            let piece = board.piece(square);
            if piece.color() != board.side_to_move() {
                continue;
            }

            let piece_gen = match piece.dignity() {
                p if p == Pawn => Self::generate_for_pawn,
                p if p == Knight => Self::generate_for_knight,
                _ => continue,
            };

            piece_gen(self, square, board, buffer);
        }
    }

    fn generate_for_pawn(
        &self,
        from: Square,
        board: &Board,
        buffer: &mut StaticBuffer<Move, MAX_MOVE_BUFFER_SIZE>,
    ) {
        let c = board.side_to_move();
        buffer.add(Move::pawn_single(from, from.forward(c, 1)));
        buffer.add(Move::pawn_double(from, from.forward(c, 2)));
    }

    fn generate_for_knight(
        &self,
        from: Square,
        board: &Board,
        buffer: &mut StaticBuffer<Move, MAX_MOVE_BUFFER_SIZE>,
    ) {
        for (dx, dy) in [
            (-2, -1),
            (-1, -2),
            (-2, 1),
            (-1, 2),
            (2, -1),
            (1, -2),
            (2, 1),
            (1, 2),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startpos_moves() {
        let board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let movegen = MoveGenerator::new();
        let mut buffer = StaticBuffer::new();

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
}
