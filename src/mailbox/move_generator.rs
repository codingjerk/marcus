use crate::prelude::*;

const MAX_MOVE_BUFFER_SIZE: usize = 500;

pub type MoveBuffer = StaticBuffer<Move, MAX_MOVE_BUFFER_SIZE>;

pub struct MoveGenerator;

impl MoveGenerator {
    #[inline(always)]
    pub const fn new() -> Self {
        Self
    }

    // PERF: try to pass color as template parameter
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
                Pawn => Self::generate_for_pawn,
                Knight => Self::generate_for_knight,
                Bishop => Self::generate_for_bishop,
                Rook => Self::generate_for_rook,
                Queen => Self::generate_for_queen,
                King => Self::generate_for_king,

                DignityNone => continue,

                _ => never!(),
            };

            piece_gen(self, square, board, buffer);
        }
    }

    pub fn make_move(
        &self,
        board: &mut Board,
        chess_move: Move,
    ) -> bool {
        let piece = if chess_move.promoted() == DignityNone {
            board.piece(chess_move.from())
        } else {
            Piece::new(board.side_to_move(), chess_move.promoted())
        };

        let stm = board.side_to_move();
        let opp_color = stm.swapped();
        let ep_to = {
            if board.en_passant_file() == FileEnPassantNone {
                None
            } else {
                Some(Square::en_passant(stm, board.en_passant_file()))
            }
        };

        if chess_move.captured() == Pawn
           && piece.dignity() == Pawn
           && Some(chess_move.to()) == ep_to
        {
            board.remove_piece(ep_to.unwrap().forward(opp_color, 1));
        } else if chess_move.captured() != DignityNone {
            always!(
                board.piece(chess_move.to()).dignity() ==
                chess_move.captured()
            );
            always!(
                board.piece(chess_move.to()).color() !=
                board.side_to_move()
            );
        }

        board.set_piece_unchecked(chess_move.to(), piece);
        board.remove_piece(chess_move.from());

        if chess_move.is_king_side_castling(piece.dignity()) {
            let cr = CastlingRights::king_side(stm);
            board.set_piece_unchecked(cr.rook_destination(), Piece::new(stm, Rook));
            board.remove_piece(cr.rook_initial());
        } else if chess_move.is_queen_side_castling(piece.dignity()) {
            let stm = board.side_to_move();
            let cr = CastlingRights::queen_side(stm);
            board.set_piece_unchecked(cr.rook_destination(), Piece::new(stm, Rook));
            board.remove_piece(cr.rook_initial());
        }

        board.push_undo();

        // Castling checks
        if piece.dignity() == King {
            board.disallow_castling(CastlingRights::both(stm));
        }

        if piece.dignity() == Rook &&
           chess_move.from().file() == FileA &&
           chess_move.from().rank() == stm.start_rank() {
            board.disallow_castling(CastlingRights::queen_side(stm));
        }

        if piece.dignity() == Rook &&
           chess_move.from().file() == FileH &&
           chess_move.from().rank() == stm.start_rank() {
            board.disallow_castling(CastlingRights::king_side(stm));
        }

        if chess_move.captured() == Rook &&
           chess_move.to().file() == FileA &&
           chess_move.to().rank() == opp_color.start_rank() {
            board.disallow_castling(CastlingRights::queen_side(opp_color));
        }

        if chess_move.captured() == Rook &&
           chess_move.to().file() == FileH &&
           chess_move.to().rank() == opp_color.start_rank() {
            board.disallow_castling(CastlingRights::king_side(opp_color));
        }

        // En-passant checks
        if chess_move.is_pawn_double_move(piece.dignity()) {
            board.set_en_passant_file(chess_move.from().file());
        } else {
            board.unset_en_passant_file();
        }

        // Halfmove clock
        if chess_move.is_capture() || piece.dignity() == Pawn {
            board.reset_halfmove_clock();
        } else {
            board.increase_halfmove_clock();
        }

        board.swap_side_to_move();

        self.was_legal(board, chess_move)
    }

    pub fn unmake_move(
        &self,
        board: &mut Board,
        chess_move: Move,
    ) {
        let moved_side = board.side_to_move();
        let opp_color = moved_side.swapped();

        if chess_move.promoted() != DignityNone {
            let pawn = Piece::new(opp_color, Pawn);
            board.set_piece(chess_move.from(), pawn);
        } else {
            let moved_piece = board.piece(chess_move.to()); // PERF: try to get from chess_move
            board.set_piece(chess_move.from(), moved_piece);
        }

        let moved_piece = board.piece(chess_move.to()); // PERF: try to get from chess_move
        board.remove_piece(chess_move.to());

        if chess_move.is_en_passant() {
            // PERF: it's always Pawn, try to hardcode
            always!(chess_move.captured() == Pawn);
            
            let captured_piece = Piece::new(moved_side, chess_move.captured());
            let en_passant_square = chess_move.to().forward(moved_side, 1);
            board.set_piece(en_passant_square, captured_piece);
        } else if chess_move.captured() != DignityNone {
            let captured_piece = Piece::new(moved_side, chess_move.captured());
            board.set_piece(chess_move.to(), captured_piece);
        }

        if chess_move.is_queen_side_castling(moved_piece.dignity()) {
            let cr = CastlingRights::queen_side(opp_color);
            board.set_piece_unchecked(cr.rook_initial(), Piece::new(opp_color, Rook));
            board.remove_piece(cr.rook_destination());
        } else if chess_move.is_king_side_castling(moved_piece.dignity()) {
            let cr = CastlingRights::king_side(opp_color);
            board.set_piece_unchecked(cr.rook_initial(), Piece::new(opp_color, Rook));
            board.remove_piece(cr.rook_destination());
        }

        board.pop_undo();
        board.swap_side_to_move();
    }

    fn was_legal(
        &self,
        board: &mut Board,
        chess_move: Move,
    ) -> bool {
        let stm = board.side_to_move();
        let moved_side = stm.swapped();

        let king_pos = board.find_king(moved_side);
        always!(king_pos.is_some());
        let king_pos = king_pos.unwrap();

        if self.can_be_attacked(king_pos, board, stm) {
            return false;
        }

        let moved_piece = board.piece(chess_move.to());
        always!(moved_piece.color() == moved_side);

        if chess_move.is_king_side_castling(moved_piece.dignity()) {
            let leave_square = king_pos.by(-2, 0).unwrap();
            if self.can_be_attacked(leave_square, board, stm) {
                return false;
            }

            let cross_square = king_pos.by(-1, 0).unwrap();
            if self.can_be_attacked(cross_square, board, stm) {
                return false;
            }
        }

        if chess_move.is_queen_side_castling(moved_piece.dignity()) {
            let leave_square = king_pos.by(2, 0).unwrap();
            if self.can_be_attacked(leave_square, board, stm) {
                return false;
            }

            let cross_square = king_pos.by(1, 0).unwrap();
            if self.can_be_attacked(cross_square, board, stm) {
                return false;
            }
        }

        true
    }

    fn can_be_attacked(
        &self,
        target: Square,
        board: &mut Board,
        side_to_move: Color,
    ) -> bool {
        let pawn = Piece::new(side_to_move, Pawn);
        let target_side = side_to_move.swapped();

        // Left pawn attack
        if let Some(attacker) = target.left_pawn_attack(target_side) {
            if board.piece(attacker) == pawn {
                return true;
            }
        }

        // Right pawn attack
        if let Some(attacker) = target.right_pawn_attack(target_side) {
            if board.piece(attacker) == pawn {
                return true;
            }
        }

        // Knight attacks
        let knight = Piece::new(side_to_move, Knight);
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
            let attacker = match target.by(dx, dy) {
                Some(s) => s,
                None => continue,
            };

            if board.piece(attacker) == knight {
                return true;
            }
        }

        // Bishop & Queen attacks
        let bishop = Piece::new(side_to_move, Bishop);
        let queen = Piece::new(side_to_move, Queen);
        for (dx, dy) in [
            (-1, -1),
            (-1,  1),
            ( 1, -1),
            ( 1,  1),
        ] {
            for d in 1..8 {
                let attacker = match target.by(dx * d, dy * d) {
                    Some(s) => s,
                    None => break,
                };

                let attacker = board.piece(attacker);
                if attacker == PieceNone {
                    // pass
                } else if attacker == bishop || attacker == queen {
                    return true;
                } else {
                    break;
                }
            }
        }

        // Rook & Queen attacks
        let rook = Piece::new(side_to_move, Rook);
        let queen = Piece::new(side_to_move, Queen);
        for (dx, dy) in [
            ( 0,  1),
            ( 0, -1),
            ( 1,  0),
            (-1,  0),
        ] {
            for d in 1..8 {
                let attacker = match target.by(dx * d, dy * d) {
                    Some(s) => s,
                    None => break,
                };

                let attacker = board.piece(attacker);
                if attacker == PieceNone {
                    // pass
                } else if attacker == rook || attacker == queen {
                    return true;
                } else {
                    break;
                }
            }
        }

        // King attacks
        let king = Piece::new(side_to_move, King);
        for (dx, dy) in [
            (-1, -1),
            (-1,  0),
            (-1,  1),
            ( 0, -1),
            ( 0,  1),
            ( 1, -1),
            ( 1,  0),
            ( 1,  1),
        ] {
            let attacker = match target.by(dx, dy) {
                Some(s) => s,
                None => continue,
            };

            if board.piece(attacker) == king {
                return true;
            }
        }

        false
    }

    fn generate_for_pawn(
        &self,
        from: Square,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        self.generate_quiets_for_pawn(from, board, buffer);
        self.generate_capture_for_pawn(from,  1, board, buffer);
        self.generate_capture_for_pawn(from, -1, board, buffer);
        self.generate_promotions_for_pawn(from, board, buffer);
        self.generate_promotion_captures_for_pawn(from,  1, board, buffer);
        self.generate_promotion_captures_for_pawn(from, -1, board, buffer);
    }

    fn generate_quiets_for_pawn(
        &self,
        from: Square,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        let stm = board.side_to_move();
        if from.rank() == Rank::pawn_pre_promotion_rank(stm) {
            return;
        }

        let to = from.forward(stm, 1);
        if board.piece(to) != PieceNone { return }
        buffer.add(Move::pawn_single(from, to));

        let to = from.forward(stm, 2);
        if board.piece(to) != PieceNone { return }
        if from.rank() != Rank::pawn_double_rank(stm) { return }

        buffer.add(Move::pawn_double(from, to));
    }

    fn generate_capture_for_pawn(
        &self,
        from: Square,
        direction: i8,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        let stm = board.side_to_move();
        if from.rank() == Rank::pawn_pre_promotion_rank(stm) {
            return;
        }

        let to = if let Some(to) = from.forward(stm, 1).by(direction, 0) {
            to
        } else {
            return;
        };

        let dest = board.piece(to);
        if dest != PieceNone && dest.color() != stm {
            buffer.add(Move::capture(from, to, dest.dignity()));
        }

        if board.en_passant_file() == FileEnPassantNone {
            return;
        }

        let ep_to = Square::en_passant(stm, board.en_passant_file());
        always!(board.piece(ep_to) == PieceNone);

        if ep_to == to {
            buffer.add(Move::en_passant(from, to));
        }
    }

    fn generate_promotions_for_pawn(
        &self,
        from: Square,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        let stm = board.side_to_move();
        if from.rank() != Rank::pawn_pre_promotion_rank(stm) {
            return;
        }

        let to = from.forward(stm, 1);
        if board.piece(to) != PieceNone { return }

        buffer.add(Move::promotion(from, to, Knight));
        buffer.add(Move::promotion(from, to, Bishop));
        buffer.add(Move::promotion(from, to, Rook));
        buffer.add(Move::promotion(from, to, Queen));
    }

    fn generate_promotion_captures_for_pawn(
        &self,
        from: Square,
        direction: i8,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        let stm = board.side_to_move();
        if from.rank() != Rank::pawn_pre_promotion_rank(stm) {
            return;
        }

        let to = if let Some(to) = from.forward(stm, 1).by(direction, 0) {
            to
        } else {
            return;
        };

        let dest = board.piece(to);
        if dest == PieceNone || dest.color() == stm {
            return;
        }

        buffer.add(Move::promotion_capture(from, to, dest.dignity(), Knight));
        buffer.add(Move::promotion_capture(from, to, dest.dignity(), Bishop));
        buffer.add(Move::promotion_capture(from, to, dest.dignity(), Rook));
        buffer.add(Move::promotion_capture(from, to, dest.dignity(), Queen));
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

            let dest = board.piece(to);
            if dest == PieceNone {
                buffer.add(Move::quiet(from, to));
            } else if dest.color() != board.side_to_move() {
                buffer.add(Move::capture(from, to, dest.dignity()));
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

    fn generate_for_king(
        &self,
        from: Square,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        for (dx, dy) in [
            (-1, -1),
            (-1,  0),
            (-1,  1),
            ( 0, -1),
            ( 0,  1),
            ( 1, -1),
            ( 1,  0),
            ( 1,  1),
        ] {
            let to = match from.by(dx, dy) {
                Some(s) => s,
                None => continue,
            };

            let dest = board.piece(to);
            if dest == PieceNone {
                buffer.add(Move::quiet(from, to));
            } else if dest.color() != board.side_to_move() {
                buffer.add(Move::capture(from, to, dest.dignity()));
            }
        }

        let stm = board.side_to_move();
        if from == Square::king_initial(stm) {
            self.generate_king_castling(board, buffer);
            self.generate_queen_castling(board, buffer);
        }
    }

    fn generate_king_castling(
        &self,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        let stm = board.side_to_move();
        let cr = CastlingRights::king_side(stm);
        if !board.castling_rights().is_allowed(cr) {
            return;
        }

        let to = cr.king_destination();
        if board.piece(to) != PieceNone {
            return;
        }

        let rook_from = cr.rook_initial();
        if board.piece(rook_from) != Piece::new(stm, Rook) {
            return;
        }

        let rook_to = cr.rook_destination();
        if board.piece(rook_to) != PieceNone {
            return;
        }

        buffer.add(Move::king_side_castling(Square::king_initial(stm), to));
    }

    fn generate_queen_castling(
        &self,
        board: &Board,
        buffer: &mut MoveBuffer,
    ) {
        let stm = board.side_to_move();
        let cr = CastlingRights::queen_side(stm);
        if !board.castling_rights().is_allowed(cr) {
            return;
        }

        let to = cr.king_destination();
        if board.piece(to) != PieceNone {
            return;
        }

        let rook_from = cr.rook_initial();
        if board.piece(rook_from) != Piece::new(stm, Rook) {
            return;
        }

        let next_to_rook = unsafe { rook_from.by(1, 0).unwrap_unchecked() };
        if board.piece(next_to_rook) != PieceNone {
            return;
        }

        let rook_to = cr.rook_destination();
        if board.piece(rook_to) != PieceNone {
            return;
        }

        buffer.add(Move::queen_side_castling(Square::king_initial(stm), to));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate(fen: &[u8]) -> MoveBuffer {
        let board = Board::from_fen(fen);
        let movegen = MoveGenerator::new();
        let mut buffer = MoveBuffer::new();
        movegen.generate(&board, &mut buffer);

        buffer
    }

    #[test]
    fn startpos() {
        let buffer = generate(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

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
    fn pawn_blocks() {
        let buffer = generate(b"8/4p3/3pPp2/p1pP1Pp1/PpP3P1/1P5p/7P/8 w - - 0 1");

        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn pawn_doubles() {
        let buffer = generate(b"8/8/5P2/4P3/3P4/2P5/1P6/8 w - - 0 1");

        assert!(buffer.contains(Move::pawn_double(b2, b4)));
        assert!(buffer.contains(Move::pawn_single(b2, b3)));
        assert!(buffer.contains(Move::pawn_single(c3, c4)));
        assert!(buffer.contains(Move::pawn_single(d4, d5)));
        assert!(buffer.contains(Move::pawn_single(e5, e6)));
        assert!(buffer.contains(Move::pawn_single(f6, f7)));

        assert_eq!(buffer.len(), 6);
    }

    #[test]
    fn pawn_captures() {
        let buffer = generate(b"8/8/8/2ppp3/3P4/8/8/8 w - - 0 1");

        assert!(buffer.contains(Move::capture(d4, e5, Pawn)));
        assert!(buffer.contains(Move::capture(d4, c5, Pawn)));

        assert_eq!(buffer.len(), 2);
    }

    #[test]
    fn pawn_en_passant() {
        let buffer = generate(b"8/8/8/4PpP1/8/8/8/8 w - f6 0 1");

        assert!(buffer.contains(Move::quiet(e5, e6)));
        assert!(buffer.contains(Move::quiet(g5, g6)));
        assert!(buffer.contains(Move::en_passant(e5, f6)));
        assert!(buffer.contains(Move::en_passant(g5, f6)));

        assert_eq!(buffer.len(), 4);
    }

    #[test]
    fn pawn_promotions() {
        let buffer = generate(b"8/3P4/8/8/8/8/8/8 w - - 0 1");

        assert!(buffer.contains(Move::promotion(d7, d8, Knight)));
        assert!(buffer.contains(Move::promotion(d7, d8, Bishop)));
        assert!(buffer.contains(Move::promotion(d7, d8, Rook)));
        assert!(buffer.contains(Move::promotion(d7, d8, Queen)));

        assert_eq!(buffer.len(), 4);
    }

    #[test]
    fn pawn_promotion_captures() {
        let buffer = generate(b"2nkn3/3P4/8/8/8/8/8/8 w - - 0 1");

        assert!(buffer.contains(Move::promotion_capture(d7, c8, Knight, Knight)));
        assert!(buffer.contains(Move::promotion_capture(d7, c8, Knight, Bishop)));
        assert!(buffer.contains(Move::promotion_capture(d7, c8, Knight, Rook)));
        assert!(buffer.contains(Move::promotion_capture(d7, c8, Knight, Queen)));

        assert!(buffer.contains(Move::promotion_capture(d7, e8, Knight, Knight)));
        assert!(buffer.contains(Move::promotion_capture(d7, e8, Knight, Bishop)));
        assert!(buffer.contains(Move::promotion_capture(d7, e8, Knight, Rook)));
        assert!(buffer.contains(Move::promotion_capture(d7, e8, Knight, Queen)));

        assert_eq!(buffer.len(), 8);
    }

    #[test]
    fn black_pawns() {
        let buffer = generate(b"8/pp6/R7/8/3pP3/8/2p5/3N4 b - e3 0 1");

        assert!(buffer.contains(Move::pawn_single(b7, b6)));
        assert!(buffer.contains(Move::pawn_double(b7, b5)));
        assert!(buffer.contains(Move::capture(b7, a6, Rook)));

        assert!(buffer.contains(Move::promotion(c2, c1, Knight)));
        assert!(buffer.contains(Move::promotion(c2, c1, Bishop)));
        assert!(buffer.contains(Move::promotion(c2, c1, Rook)));
        assert!(buffer.contains(Move::promotion(c2, c1, Queen)));

        assert!(buffer.contains(Move::promotion_capture(c2, d1, Knight, Knight)));
        assert!(buffer.contains(Move::promotion_capture(c2, d1, Knight, Bishop)));
        assert!(buffer.contains(Move::promotion_capture(c2, d1, Knight, Rook)));
        assert!(buffer.contains(Move::promotion_capture(c2, d1, Knight, Queen)));

        assert!(buffer.contains(Move::pawn_single(d4, d3)));
        assert!(buffer.contains(Move::en_passant(d4, e3)));

        assert_eq!(buffer.len(), 13);
    }

    #[test]
    fn king_side_castling() {
        let buffer = generate(b"8/8/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1");

        assert!(buffer.contains(Move::king_side_castling(e1, g1)));
        assert_eq!(buffer.len(), 22);

        let buffer = generate(b"8/8/8/8/8/8/PPPPPPPP/RNBQK2R w Qkq - 0 1");

        assert!(!buffer.contains(Move::king_side_castling(e1, g1)));
        assert_eq!(buffer.len(), 21);

        let buffer = generate(b"8/8/8/8/8/8/PPPPPPPP/RNBQK3 w KQkq - 0 1");

        assert!(!buffer.contains(Move::king_side_castling(e1, g1)));
        assert_eq!(buffer.len(), 19);
    }

    #[test]
    fn queen_side_castling() {
        let buffer = generate(b"8/8/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1");
        assert!(buffer.contains(Move::queen_side_castling(e1, c1)));

        for fen_no_castling in [
            &b"8/8/8/8/8/8/PPPPPPPP/R3KBNR w Kkq - 0 1"[..],
            &b"8/8/8/8/8/8/PPPPPPPP/4KBNR w KQkq - 0 1"[..],
            &b"8/8/8/8/8/8/PPPPPPPP/RN2KBNR w KQkq - 0 1"[..],
            &b"8/8/8/8/8/8/PPPPPPPP/R1B1KBNR w KQkq - 0 1"[..],
            &b"8/8/8/8/8/8/PPPPPPPP/R2QKBNR w KQkq - 0 1"[..],
        ] {
            let buffer = generate(fen_no_castling);
            assert!(!buffer.contains(Move::queen_side_castling(e1, c1)));
        }
    }

    #[test]
    fn knights() {
        let buffer = generate(b"8/8/4p3/8/1p1N4/1P6/8/8 w - - 0 1");

        assert!(buffer.contains(Move::quiet(d4, f5)));
        assert!(buffer.contains(Move::quiet(d4, f3)));
        assert!(buffer.contains(Move::quiet(d4, e2)));
        assert!(buffer.contains(Move::quiet(d4, c2)));
        assert!(buffer.contains(Move::quiet(d4, b5)));
        assert!(buffer.contains(Move::quiet(d4, c6)));
        assert!(buffer.contains(Move::capture(d4, e6, Pawn)));

        assert_eq!(buffer.len(), 7);
    }

    #[test]
    fn bishop() {
        let buffer = generate(b"8/8/5r2/8/3B4/8/5B2/8 w - - 0 1");

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
        let buffer = generate(b"8/8/8/1n6/8/8/1R4R1/8 w - - 0 1");

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
        let buffer = generate(b"8/n7/3p1b2/8/1k1Q1r2/2q1n3/3p4/8 w - - 0 1");

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

    #[test]
    fn king() {
        let buffer = generate(b"8/8/8/8/5n2/4K3/8/8 w - - 0 1");

        assert!(buffer.contains(Move::quiet(e3, f3)));
        assert!(buffer.contains(Move::quiet(e3, f2)));
        assert!(buffer.contains(Move::quiet(e3, e2)));
        assert!(buffer.contains(Move::quiet(e3, d2)));
        assert!(buffer.contains(Move::quiet(e3, d3)));
        assert!(buffer.contains(Move::quiet(e3, d4)));
        assert!(buffer.contains(Move::quiet(e3, e4)));
        assert!(buffer.contains(Move::capture(e3, f4, Knight)));

        assert_eq!(buffer.len(), 8);
    }

    #[test]
    fn fuzz_generation() {
        let mut rng = FastRng::from_system_time();
        let movegen = MoveGenerator::new();
        let mut buffer = MoveBuffer::new();

        for i in 0..(11_010 * FUZZ_MULTIPLIER) {
            let board = Board::rand(&mut rng);
            buffer.reset();

            if !board.has_possible_pawn_structure() ||
               !board.has_possible_en_passant_square() ||
               !board.has_possible_kings_setup()
            {
                continue;
            }

            movegen.generate(&board, &mut buffer);
        }
    }

    #[test]
    fn make_move_quiet() {
        let mut board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::quiet(e2, e4));

        assert!(legal);
        assert_eq!(board.side_to_move(), Black);
        assert_eq!(board.piece(e2), PieceNone);
        assert_eq!(board.piece(e4), WhitePawn);
    }

    #[test]
    fn make_move_promotion() {
        let mut board = Board::from_fen(b"k7/3P4/8/8/8/8/8/K7 w - - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::promotion(d7, d8, Queen));

        assert!(legal);
        assert_eq!(board.piece(d7), PieceNone);
        assert_eq!(board.piece(d8), WhiteQueen);
    }

    #[test]
    fn make_move_capture() {
        let mut board = Board::from_fen(b"k7/8/8/5r2/4P3/8/8/K7 w - - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::capture(e4, f5, Rook));

        assert!(legal);
        assert_eq!(board.piece(e4), PieceNone);
        assert_eq!(board.piece(f5), WhitePawn);
    }

    #[test]
    fn make_move_king_side_castling() {
        let mut board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::king_side_castling(e1, g1));

        assert!(legal);
        assert_eq!(board.piece(e1), PieceNone);
        assert_eq!(board.piece(h1), PieceNone);

        assert_eq!(board.piece(g1), WhiteKing);
        assert_eq!(board.piece(f1), WhiteRook);
    }

    #[test]
    fn make_move_queen_side_castling() {
        let mut board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::queen_side_castling(e1, c1));

        assert!(legal);
        assert_eq!(board.piece(e1), PieceNone);
        assert_eq!(board.piece(a1), PieceNone);

        assert_eq!(board.piece(c1), WhiteKing);
        assert_eq!(board.piece(d1), WhiteRook);
    }

    #[test]
    fn make_move_en_passant() {
        let mut board = Board::from_fen(b"k7/8/8/5Pp1/8/8/8/K7 w - g6 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::en_passant(f5, g6));

        assert!(legal);
        assert_eq!(board.piece(f5), PieceNone);
        assert_eq!(board.piece(g6), WhitePawn);

        assert_eq!(board.piece(g5), PieceNone);
    }

    #[test]
    fn make_move_legality_direct_check() {
        let mut board = Board::from_fen(b"3r4/8/8/8/8/8/8/3KN3 w - - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::quiet(e1, f3));

        assert!(!legal);
    }

    #[test]
    fn make_move_legality_pinned_piece() {
        let mut board = Board::from_fen(b"3r4/8/8/8/8/8/3N4/3K4 w - - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::quiet(d2, e4));

        assert!(!legal);
    }

    #[test]
    fn make_move_legality_en_passant_pin() {
        let mut board = Board::from_fen(b"8/8/8/2KPp2q/8/8/8/8 w - e6 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::en_passant(d5, e6));

        assert!(!legal);
    }

    #[test]
    fn make_move_legality_king_side_castling() {
        // Leave
        let mut board = Board::from_fen(b"4r3/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::king_side_castling(e1, g1));

        assert!(!legal);

        // Cross
        let mut board = Board::from_fen(b"5r2/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::king_side_castling(e1, g1));

        assert!(!legal);

        // End-up
        let mut board = Board::from_fen(b"6r1/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::king_side_castling(e1, g1));

        assert!(!legal);
    }

    #[test]
    fn make_move_legality_queen_side_castling() {
        // Leave
        let mut board = Board::from_fen(b"4r3/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::queen_side_castling(e1, c1));

        assert!(!legal);

        // Cross
        let mut board = Board::from_fen(b"3r4/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::queen_side_castling(e1, c1));

        assert!(!legal);

        // End-up
        let mut board = Board::from_fen(b"2r5/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, Move::queen_side_castling(e1, c1));

        assert!(!legal);
    }

    #[test]
    fn make_move_king_move_resets_castling_rights() {
        let mut board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
        let chess_move = Move::queen_side_castling(e1, c1);

        let movegen = MoveGenerator::new();
        let _legal = movegen.make_move(&mut board, chess_move);
        assert!(!board.castling_rights().is_allowed(WhiteKingSide));
        assert!(!board.castling_rights().is_allowed(WhiteQueenSide));
    }

    #[test]
    fn make_move_rook_move_resets_castling_rights() {
        let mut board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
        let chess_move = Move::quiet(a1, a2);

        let movegen = MoveGenerator::new();
        let _legal = movegen.make_move(&mut board, chess_move);
        assert!(board.castling_rights().is_allowed(WhiteKingSide));
        assert!(!board.castling_rights().is_allowed(WhiteQueenSide));
    }

    #[test]
    fn make_move_rook_capture_resets_castling_rights() {
        let mut board = Board::from_fen(b"rnbqkbnr/8/8/8/8/8/8/R3K2R b KQkq - 0 1");
        let chess_move = Move::capture(h8, h1, Rook);

        let movegen = MoveGenerator::new();
        let _legal = movegen.make_move(&mut board, chess_move);
        assert!(!board.castling_rights().is_allowed(WhiteKingSide));
        assert!(board.castling_rights().is_allowed(WhiteQueenSide));
    }

    #[test]
    fn make_move_resets_en_passant_square() {
        let mut board = Board::from_fen(b"rnbqkbnr/8/8/8/8/8/8/R3K2R b KQkq e3 0 1");
        let chess_move = Move::capture(h8, h1, Rook);

        let movegen = MoveGenerator::new();
        let _legal = movegen.make_move(&mut board, chess_move);

        assert_eq!(board.en_passant_file(), FileEnPassantNone);
    }

    #[test]
    fn make_move_double_pawn_sets_en_passant_square() {
        let mut board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let chess_move = Move::pawn_double(d2, d4);

        let movegen = MoveGenerator::new();
        let _legal = movegen.make_move(&mut board, chess_move);

        assert_eq!(board.en_passant_file(), FileD);
    }

    #[test]
    fn make_move_capture_resets_halfmove_clock() {
        let mut board = Board::from_fen(b"1k2r3/8/8/8/4B3/8/2P5/5K2 b - - 13 1");
        let chess_move = Move::capture(e8, e4, Bishop);
        assert_eq!(board.halfmove_clock(), 13);

        let movegen = MoveGenerator::new();
        let _legal = movegen.make_move(&mut board, chess_move);

        assert_eq!(board.halfmove_clock(), 0);
    }

    #[test]
    fn make_move_pawn_move_resets_halfmove_clock() {
        let mut board = Board::from_fen(b"1k2r3/8/8/8/4B3/8/2P5/5K2 w - - 15 1");
        let chess_move = Move::pawn_single(c2, c3);
        assert_eq!(board.halfmove_clock(), 15);

        let movegen = MoveGenerator::new();
        let _legal = movegen.make_move(&mut board, chess_move);

        assert_eq!(board.halfmove_clock(), 0);
    }

    #[test]
    fn make_move_moves_increase_halfmove_clock() {
        let mut board = Board::from_fen(b"1k2r3/8/8/8/4B3/8/2P5/5K2 b - - 15 1");
        let chess_move = Move::quiet(e8, e5);
        assert_eq!(board.halfmove_clock(), 15);

        let movegen = MoveGenerator::new();
        let _legal = movegen.make_move(&mut board, chess_move);

        assert_eq!(board.halfmove_clock(), 16);
    }

    #[test]
    fn unmake_move_restores_side_to_move() {
        let mut board = Board::from_fen(b"4k3/8/8/8/8/8/8/R3K3 w KQkq - 0 1");
        let chess_move = Move::quiet(a1, c1);
        let movegen = MoveGenerator::new();
        let _ = movegen.make_move(&mut board, chess_move);
        assert_eq!(board.side_to_move(), Black);

        movegen.unmake_move(&mut board, chess_move);
        assert_eq!(board.side_to_move(), White);
    }

    #[test]
    fn unmake_move_quiet() {
        let mut board = Board::from_fen(b"4k3/8/8/8/8/8/8/R3K3 w KQkq - 0 1");
        let chess_move = Move::quiet(a1, c1);
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, chess_move);

        assert!(legal);
        movegen.unmake_move(&mut board, chess_move);

        assert_eq!(board.piece(c1), PieceNone);
        assert_eq!(board.piece(a1), WhiteRook);
    }

    #[test]
    fn unmake_move_capture() {
        let mut board = Board::from_fen(b"4k3/8/8/8/8/8/8/R1b1K3 w KQkq - 0 1");
        let chess_move = Move::capture(a1, c1, Bishop);
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, chess_move);

        assert!(legal);
        movegen.unmake_move(&mut board, chess_move);

        assert_eq!(board.piece(c1), BlackBishop);
        assert_eq!(board.piece(a1), WhiteRook);
    }

    #[test]
    fn unmake_move_promotion() {
        let mut board = Board::from_fen(b"8/4P3/8/8/8/8/8/1K6 w - - 0 1");
        let chess_move = Move::promotion(e7, e8, Queen);
        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, chess_move);

        assert!(legal);
        movegen.unmake_move(&mut board, chess_move);

        assert_eq!(board.piece(e7), WhitePawn);
        assert_eq!(board.piece(e8), PieceNone);
    }

    #[test]
    fn unmake_move_castling_moves_rook() {
        let mut board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
        let chess_move = Move::queen_side_castling(e1, c1);

        let movegen = MoveGenerator::new();
        let legal = movegen.make_move(&mut board, chess_move);

        assert!(legal);
        movegen.unmake_move(&mut board, chess_move);

        assert_eq!(board.piece(e1), WhiteKing);
        assert_eq!(board.piece(a1), WhiteRook);
        assert_eq!(board.piece(c1), PieceNone);
        assert_eq!(board.piece(d1), PieceNone);

        let chess_move = Move::king_side_castling(e1, g1);
        let legal = movegen.make_move(&mut board, chess_move);

        assert!(legal);
        movegen.unmake_move(&mut board, chess_move);

        assert_eq!(board.piece(e1), WhiteKing);
        assert_eq!(board.piece(h1), WhiteRook);
        assert_eq!(board.piece(g1), PieceNone);
        assert_eq!(board.piece(f1), PieceNone);
    }

    #[test]
    fn unmake_move_restores_castling_rights() {
        let mut board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1");
        let chess_move = Move::queen_side_castling(e1, c1);

        let movegen = MoveGenerator::new();
        let _legal = movegen.make_move(&mut board, chess_move);
        movegen.unmake_move(&mut board, chess_move);

        assert!(board.castling_rights().is_allowed(WhiteKingSide));
        assert!(board.castling_rights().is_allowed(WhiteQueenSide));
    }

    #[test]
    fn unmake_move_restores_en_passant_captured_pawn() {
        let mut board = Board::from_fen(b"k7/8/8/5Pp1/8/8/8/K7 w - g6 0 1");
        let movegen = MoveGenerator::new();
        let chess_move = Move::en_passant(f5, g6);
        let _legal = movegen.make_move(&mut board, Move::en_passant(f5, g6));
        movegen.unmake_move(&mut board, chess_move);

        assert_eq!(board.piece(g5), BlackPawn);
    }

    #[test]
    fn unmake_move_restores_en_passant_square() {
        let mut board = Board::from_fen(b"k7/8/8/5Pp1/8/8/8/K7 w - g6 0 1");
        let movegen = MoveGenerator::new();
        let chess_move = Move::en_passant(f5, g6);
        let _legal = movegen.make_move(&mut board, Move::en_passant(f5, g6));
        movegen.unmake_move(&mut board, chess_move);

        assert_eq!(board.en_passant_file(), FileG);
    }

    #[test]
    fn unmake_move_restores_halfmove_clock() {
        let mut board = Board::from_fen(b"1k2r3/8/8/8/4B3/8/2P5/5K2 b - - 13 1");
        let chess_move = Move::capture(e8, e4, Bishop);
        let movegen = MoveGenerator::new();
        let _legal = movegen.make_move(&mut board, chess_move);
        movegen.unmake_move(&mut board, chess_move);

        assert_eq!(board.halfmove_clock(), 13);
    }

    #[test]
    fn fuzz_make_unmake() {
        let mut rng = FastRng::from_system_time();
        // STYLE: rename to move_gen or move_generator
        let movegen = MoveGenerator::new();
        let mut buffer = MoveBuffer::new();

        for i in 0..(11_010 * FUZZ_MULTIPLIER) {
            let mut board = Board::rand(&mut rng);
            buffer.reset();

            if !board.has_possible_pawn_structure() ||
               !board.has_possible_en_passant_square() ||
               !board.has_possible_kings_setup()
            {
                continue;
            }

            movegen.generate(&board, &mut buffer);

            // TODO: use iterator
            for i in 0..buffer.len() {
                let chess_move = buffer.get(i);
                let _legal = movegen.make_move(&mut board, chess_move);
                movegen.unmake_move(&mut board, chess_move);
            }
        }
    }
}

#[cfg(test)]
mod bench {
    use super::*;

    use test::{Bencher, black_box};

    macro_rules! bench_generate {
        ($b:ident, $fen:literal) => {
            let fen = black_box($fen);
            let board = Board::from_fen(fen);
            let movegen = MoveGenerator::new();

            $b.iter(|| {
                let mut buffer = MoveBuffer::new();
                movegen.generate(&board, &mut buffer);

                buffer
            })
        }
    }

    #[bench]
    fn generate_empty(b: &mut Bencher) {
        bench_generate!(b, b"8/8/8/8/8/8/8/8 w KQkq - 0 1");
    }

    #[bench]
    fn generate_startpos(b: &mut Bencher) {
        bench_generate!(b, b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    #[bench]
    fn generate_pawns(b: &mut Bencher) {
        bench_generate!(b, b"8/pppppppp/8/8/8/8/PPPPPPPP/8 w KQkq - 0 1");
    }

    #[bench]
    fn generate_knights(b: &mut Bencher) {
        bench_generate!(b, b"8/8/nnnnnnnn/8/8/NNNNNNNN/8/8 w KQkq - 0 1");
    }

    #[bench]
    fn generate_bishops(b: &mut Bencher) {
        bench_generate!(b, b"8/8/bbbbbbbb/8/8/BBBBBBBB/8/8 w KQkq - 0 1");
    }

    #[bench]
    fn generate_rooks(b: &mut Bencher) {
        bench_generate!(b, b"8/8/rrrrrrrr/8/8/RRRRRRRR/8/8 w KQkq - 0 1");
    }

    #[bench]
    fn generate_queens(b: &mut Bencher) {
        bench_generate!(b, b"8/8/qqqqqqqq/8/8/QQQQQQQQ/8/8 w KQkq - 0 1");
    }

    #[bench]
    fn generate_kings(b: &mut Bencher) {
        bench_generate!(b, b"8/8/kkkkkkkk/8/8/KKKKKKKK/8/8 w KQkq - 0 1");
    }

    #[bench]
    fn generate_empty_black(b: &mut Bencher) {
        bench_generate!(b, b"8/8/8/8/8/8/8/8 b KQkq - 0 1");
    }

    #[bench]
    fn generate_startpos_black(b: &mut Bencher) {
        bench_generate!(b, b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
    }

    #[bench]
    fn generate_pawns_black(b: &mut Bencher) {
        bench_generate!(b, b"8/pppppppp/8/8/8/8/PPPPPPPP/8 b KQkq - 0 1");
    }

    #[bench]
    fn generate_knights_black(b: &mut Bencher) {
        bench_generate!(b, b"8/8/nnnnnnnn/8/8/NNNNNNNN/8/8 b KQkq - 0 1");
    }

    #[bench]
    fn generate_bishops_black(b: &mut Bencher) {
        bench_generate!(b, b"8/8/bbbbbbbb/8/8/BBBBBBBB/8/8 b KQkq - 0 1");
    }

    #[bench]
    fn generate_rooks_black(b: &mut Bencher) {
        bench_generate!(b, b"8/8/rrrrrrrr/8/8/RRRRRRRR/8/8 b KQkq - 0 1");
    }

    #[bench]
    fn generate_queens_black(b: &mut Bencher) {
        bench_generate!(b, b"8/8/qqqqqqqq/8/8/QQQQQQQQ/8/8 b KQkq - 0 1");
    }

    #[bench]
    fn generate_kings_black(b: &mut Bencher) {
        bench_generate!(b, b"8/8/kkkkkkkk/8/8/KKKKKKKK/8/8 b KQkq - 0 1");
    }
}
