use crate::prelude::*;

// PERF: try other types
type Depth = usize;

fn perft_recursive(
    board: &mut Board,
    movegen: &MoveGenerator,
    move_buffer: &mut MoveBuffer,
    depth: Depth,
) -> usize {
    if depth == 0 {
        return 1;
    }

    let start_move_index = move_buffer.len();
    movegen.generate(board, move_buffer);
    let end_move_index = move_buffer.len();

    let mut result = 0;
    for move_index in start_move_index..end_move_index {
        let chess_move = move_buffer.get(move_index);
        let legal = movegen.make_move(board, chess_move);
        if legal {
            result += perft_recursive(board, movegen, move_buffer, depth - 1);
        }

        movegen.unmake_move(board, chess_move);
    }

    move_buffer.restore_cursor(start_move_index);

    result
}

fn perft(fen: &[u8], depth: Depth) -> usize {
    let mut board = Board::from_fen(fen);
    let movegen = MoveGenerator::new();
    let mut move_buffer = MoveBuffer::new();

    perft_recursive(
        &mut board,
        &movegen,
        &mut move_buffer,
        depth,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: all positions and results here are from
    // https://www.chessprogramming.org/Perft_Results

    #[test]
    fn startpos() {
        let fen = b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        for (result, depth, fen) in [
            (20, 1, fen),
            (400, 2, fen),
            (8_902, 3, fen),
            (197_281, 4, fen),
            (4_865_609, 5, fen),
        ] {
            assert_eq!(perft(fen, depth), result);
        }
    }

    #[test]
    fn kiwipete() {
        let fen = b"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        for (result, depth, fen) in [
            (48, 1, fen),
            (2039, 2, fen),
            (97_862, 3, fen),
            (4_085_603, 4, fen),
        ] {
            assert_eq!(perft(fen, depth), result);
        }
    }

    #[test]
    fn cpw_position_3() {
        let fen = b"8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
        for (result, depth, fen) in [
            (14, 1, fen),
            (191, 2, fen),
            (2_812, 3, fen),
            (43_238, 4, fen),
            (674_624, 5, fen),
        ] {
            assert_eq!(perft(fen, depth), result);
        }
    }

    #[test]
    fn cpw_position_4() {
        let fen = b"r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
        for (result, depth, fen) in [
            (6, 1, fen),
            (264, 2, fen),
            (9_467, 3, fen),
            (422_333, 4, fen),
            (15_833_292, 5, fen),
        ] {
            assert_eq!(perft(fen, depth), result);
        }
    }

    #[test]
    fn cpw_position_5() {
        let fen = b"rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        for (result, depth, fen) in [
            (44, 1, fen),
            (1_486, 2, fen),
            (62_379, 3, fen),
            (2_103_487, 4, fen),
        ] {
            assert_eq!(perft(fen, depth), result);
        }
    }

    #[test]
    fn cpw_position_6() {
        let fen = b"r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
        for (result, depth, fen) in [
            (46, 1, fen),
            (2_079, 2, fen),
            (89_890, 3, fen),
            (3_894_594, 4, fen),
        ] {
            assert_eq!(perft(fen, depth), result);
        }
    }
}
