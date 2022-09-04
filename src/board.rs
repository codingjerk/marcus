use crate::prelude::*;

type HalfmoveClock = u16; // PERF: try smaller and bigger types
const MAX_HALFMOVE_CLOCK: HalfmoveClock = 999;
const MIN_FEN_SIZE: usize = 24;
const MAX_FEN_SIZE: usize = 90;

// TODO: move to board/mailbox8x8
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

        let mut result = unsafe { undefined() };

        result
    }
}
