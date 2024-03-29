use crate::prelude::*;

// PERF: try other types
type Depth = usize;

fn perft_recursive<const TT_SIZE: usize>(
    board: &mut Board,
    movegen: &MoveGenerator,
    move_buffer: &mut MoveBuffer,
    transposition_table: &mut TranspositionTable<TT_SIZE>,
    depth: Depth,
) -> usize {
    if depth == 0 {
        return 1;
    }

    if let Some(nodes) = transposition_table.get(board, depth) {
        // TODO: hash-match / missmatch statistics
        return nodes;
    };

    // TODO: find a better way to track depth
    let start_move_index = move_buffer.len();
    movegen.generate(board, move_buffer);
    let end_move_index = move_buffer.len();

    let mut result = 0;
    // TODO: use iterators
    for move_index in start_move_index..end_move_index {
        let chess_move = move_buffer.get(move_index);
        let legal = movegen.make_move(board, chess_move);
        if legal {
            result += perft_recursive(board, movegen, move_buffer, transposition_table, depth - 1);
        }

        movegen.unmake_move(board, chess_move);
    }

    move_buffer.restore_cursor(start_move_index);

    transposition_table.add(board, depth, result);

    result
}

pub fn perft(fen: &[u8], depth: Depth) -> usize {
    let mut board = Board::from_fen(fen);
    let movegen = MoveGenerator::new();
    let mut move_buffer = MoveBuffer::new();
    let mut transposition_table = TranspositionTable::<{512 * 1024}>::new_box();

    let result = perft_recursive(
        &mut board,
        &movegen,
        &mut move_buffer,
        &mut transposition_table,
        depth,
    );

    #[cfg(feature = "transposition_table_stats")]
    {
        transposition_table.print_statistics();
    }

    #[allow(clippy::let_and_return)]
    result
}

pub fn perft_threaded(fen: &[u8], depth: Depth) -> usize {
    let mut board = Board::from_fen(fen);
    let movegen = MoveGenerator::new();
    let mut move_buffer = MoveBuffer::new();
    let mut threads = Vec::with_capacity(16);

    movegen.generate(&board, &mut move_buffer);

    for move_index in 0..move_buffer.len() {
        let chess_move = move_buffer.get(move_index);
        let legal = movegen.make_move(&mut board, chess_move);
        if legal {
            let mut child_board = board.clone();
            threads.push(std::thread::spawn(move || {
                let child_movegen = MoveGenerator::new();
                let mut child_move_buffer = MoveBuffer::new();
                let mut child_transposition_table = TranspositionTable::<{512 * 1024}>::new_box();

                let result = perft_recursive(
                    &mut child_board,
                    &child_movegen,
                    &mut child_move_buffer,
                    &mut child_transposition_table,
                    depth - 1,
                );

                #[cfg(feature = "transposition_table_stats")]
                child_transposition_table.print_statistics();

                #[allow(clippy::let_and_return)]
                result
            }));
        }

        movegen.unmake_move(&mut board, chess_move);
    }

    let mut result = 0;
    for thread in threads {
        result += thread.join().unwrap();
    }

    result
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

#[cfg(test)]
mod bench {
    use super::*;

    use test::{Bencher, black_box};

    macro_rules! bench_perft {
        ($b:ident, $fen:literal, $depth:literal) => {
            let fen = black_box($fen);
            let depth = black_box($depth);

            $b.iter(|| {
                perft($fen, $depth)
            })
        }
    }

    #[bench]
    fn perft_startpos(b: &mut Bencher) {
        bench_perft!(b, b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 4);
    }

    #[bench]
    fn perft_kiwipete(b: &mut Bencher) {
        bench_perft!(b, b"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", 3);
    }

    #[bench]
    fn perft_cpw_position_3(b: &mut Bencher) {
        bench_perft!(b, b"8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 4);
    }

    #[bench]
    fn perft_cpw_position_4(b: &mut Bencher) {
        bench_perft!(b, b"r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 4);
    }

    #[bench]
    fn perft_cpw_position_5(b: &mut Bencher) {
        bench_perft!(b, b"rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", 3);
    }

    #[bench]
    fn perft_cpw_position_6(b: &mut Bencher) {
        bench_perft!(b, b"r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", 3);
    }
}
