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
    }
}
