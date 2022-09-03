const assert = @import("std").debug.assert;
const std = @import("std");

const CastlingRights = @import("castling.zig").Rights;
const Color = @import("color.zig").Color;
const Piece = @import("piece.zig").Piece;
const Square = @import("square.zig").Square;

const HalfmoveClock = u10; // max is 1023
const FullmoveCounter = u14; // max is 16383

pub const Board = struct {
    squares: [64] Piece,
    sideToMove: Color,
    halfmoveClock: HalfmoveClock,

    // UndoList
    castlingRights: CastlingRights, // 4 bits
    enpassantSquare: ?Square, // 7 bits if Square, 4 bits if File
    // PERF: try to add something like "invalid enpassant square"
    //       to keep it 6 bytes, or use File representation

    const Self = *const @This();
    const MutSelf = * @This();

    const MIN_FEN_SIZE = 24;
    const MAX_FEN_SIZE = 90;

    pub fn empty() Board {
        return Board {
            .squares = [_]Piece{Piece.None} ** 64,
            .sideToMove = Color.White,
            .castlingRights = CastlingRights.None,
            .enpassantSquare = null,
            .halfmoveClock = 0,
        };
    }

    pub fn fromFen(fen: []const u8) Board {
        assert(fen.len >= MIN_FEN_SIZE);
        assert(fen.len <= MAX_FEN_SIZE);

        var result = Board {
            .squares = undefined,
            .sideToMove = undefined,
            .castlingRights = undefined,
            .enpassantSquare = undefined,
            .halfmoveClock = undefined,
        };
        
        var fenIndex: u7 = 0;

        // 1. Position
        var squareIndex = Square.A8;
        while (true) : (fenIndex += 1) {
            assert(fenIndex < fen.len);

            switch (fen[fenIndex]) {
                '1' ... '8' => |skipChar| {
                    const skip = (skipChar - '0');
                    assert(skip >= 1);
                    assert(skip <= 8);

                    var i: u4 = 0;
                    while (i < skip) : (i += 1) {
                        result.setPieceUnchecked(squareIndex, Piece.None);
                        squareIndex.moveRightUnchecked(1);
                    }
                },
                '/' => {
                    squareIndex.moveDownUnchecked(2);
                },
                ' ' => {
                    assert(squareIndex.equals(Square.A2));
                    fenIndex += 1;
                    break;
                },
                else => |pieceChar| {
                    result.setPieceUnchecked(squareIndex, Piece.fromFen(pieceChar));
                    squareIndex.moveRightUnchecked(1);
                },
            }
        }

        // 2. Side to move
        result.sideToMove = switch (fen[fenIndex]) {
            'b' => Color.Black,
            'w' => Color.White,
            else => unreachable,
        };
        fenIndex += 1;

        // Skip space
        assert(fen[fenIndex] == ' ');
        fenIndex += 1;

        // 3. Castling rights
        result.castlingRights.unset();
        while (true) : (fenIndex += 1) {
            assert(fenIndex < fen.len);

            switch (fen[fenIndex]) {
                ' ' => {
                    fenIndex += 1;
                    break;
                },
                '-' => {
                    fenIndex += 1;
                    assert(fen[fenIndex] == ' ');
                    fenIndex += 1;
                    break;
                },
                else => |char| {
                    result.castlingRights.setFromFen(char);
                },
            }
        }

        // 4. En passant target square
        if (fen[fenIndex] == '-') {
            result.enpassantSquare = null;
            fenIndex += 1;
            assert(fen[fenIndex] == ' ');
            fenIndex += 1;
        } else {
            const file = fen[fenIndex];
            fenIndex += 1;
            const rank = fen[fenIndex];
            fenIndex += 1;
            assert(fen[fenIndex] == ' ');
            fenIndex += 1;

            if (result.sideToMove.equals(Color.White)) {
                assert(rank == '6');
            } else {
                assert(rank == '3');
            }

            result.enpassantSquare = Square.fromFen(file, rank);
        }

        // 5. Halfmove clock
        result.halfmoveClock = 0;
        while (true) : (fenIndex += 1) {
            assert(fenIndex < fen.len);

            if (fen[fenIndex] == ' ') {
                fenIndex += 1;
                break;
            }

            result.halfmoveClock *= 10;
            result.halfmoveClock += (fen[fenIndex] - '0');
        }

        // 6. Fullmove counter
        // NOTE: fullmove counter is ignored

        return result;
    }

    pub inline fn setPieceUnchecked(
        self: MutSelf,
        at: Square,
        piece: Piece,
    ) void {
        const index = at.getIndex();
        assert(index <= Square.Max.getIndex());

        self.squares[index] = piece;
    }

    pub inline fn setEnpassantSquare(self: MutSelf, square: ?Square) void {
        if (self.sideToMove == Color.White) {
            assert(square == square);
        }
    }

    pub inline fn getPiece(self: Self, at: Square) Piece {
        const index = at.getIndex();
        assert(index <= Square.Max.getIndex());

        return self.squares[index];
    }

    pub inline fn getSideToMove(self: Self) Color {
        return self.sideToMove;
    }

    pub inline fn getCastlingRights(self: Self) CastlingRights {
        return self.castlingRights;
    }

    pub inline fn getEnpassantSquare(self: Self) ?Square {
        return self.enpassantSquare;
    }

    pub inline fn getHalfmoveClock(self: Self) HalfmoveClock {
        return self.halfmoveClock;
    }

    pub inline fn getFullmoveCounter(_: Self) FullmoveCounter {
        return 1;
    }
};

const expectEqual = @import("std").testing.expectEqual;

test "fromFen empty" {
    const board = Board.fromFen("8/8/8/8/8/8/8/8 w - - 0 1");
    try expectEqual(Board.empty(), board);
    
    try expectEqual(Piece.None, board.getPiece(Square.A1));
}

test "fromFen starting" {
    const board = Board.fromFen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    
    try expectEqual(Piece.WhiteRook,   board.getPiece(Square.A1));
    try expectEqual(Piece.WhiteKnight, board.getPiece(Square.B1));
    try expectEqual(Piece.WhiteBishop, board.getPiece(Square.C1));
    try expectEqual(Piece.WhiteQueen,  board.getPiece(Square.D1));
    try expectEqual(Piece.WhiteKing,   board.getPiece(Square.E1));
    try expectEqual(Piece.WhiteBishop, board.getPiece(Square.F1));
    try expectEqual(Piece.WhiteKnight, board.getPiece(Square.G1));
    try expectEqual(Piece.WhiteRook,   board.getPiece(Square.H1));
    
    try expectEqual(Piece.WhitePawn, board.getPiece(Square.A2));
    try expectEqual(Piece.WhitePawn, board.getPiece(Square.B2));
    try expectEqual(Piece.WhitePawn, board.getPiece(Square.C2));
    try expectEqual(Piece.WhitePawn, board.getPiece(Square.D2));
    try expectEqual(Piece.WhitePawn, board.getPiece(Square.E2));
    try expectEqual(Piece.WhitePawn, board.getPiece(Square.F2));
    try expectEqual(Piece.WhitePawn, board.getPiece(Square.G2));
    try expectEqual(Piece.WhitePawn, board.getPiece(Square.H2));

    try expectEqual(Piece.BlackPawn, board.getPiece(Square.A7));
    try expectEqual(Piece.BlackPawn, board.getPiece(Square.B7));
    try expectEqual(Piece.BlackPawn, board.getPiece(Square.C7));
    try expectEqual(Piece.BlackPawn, board.getPiece(Square.D7));
    try expectEqual(Piece.BlackPawn, board.getPiece(Square.E7));
    try expectEqual(Piece.BlackPawn, board.getPiece(Square.F7));
    try expectEqual(Piece.BlackPawn, board.getPiece(Square.G7));
    try expectEqual(Piece.BlackPawn, board.getPiece(Square.H7));
    
    try expectEqual(Piece.BlackRook,   board.getPiece(Square.A8));
    try expectEqual(Piece.BlackKnight, board.getPiece(Square.B8));
    try expectEqual(Piece.BlackBishop, board.getPiece(Square.C8));
    try expectEqual(Piece.BlackQueen,  board.getPiece(Square.D8));
    try expectEqual(Piece.BlackKing,   board.getPiece(Square.E8));
    try expectEqual(Piece.BlackBishop, board.getPiece(Square.F8));
    try expectEqual(Piece.BlackKnight, board.getPiece(Square.G8));
    try expectEqual(Piece.BlackRook,   board.getPiece(Square.H8));
}

fn FenTestCase(comptime E: type) type {
    return struct {
        fen: []const u8,
        expected: E,
    };
}

test "fromFen sideToMove" {
    for ([_]FenTestCase(Color){
        .{ .fen = "8/8/8/8/8/8/8/8 w - - 0 1", .expected = Color.White },
        .{ .fen = "8/8/8/8/8/8/8/8 b - - 0 1", .expected = Color.Black },
    }) |case| {
        const board = Board.fromFen(case.fen);
        try expectEqual(case.expected, board.getSideToMove());
    }
}

test "fromFen castling" {
    for ([_]FenTestCase(CastlingRights){
        .{
            .fen = "8/8/8/8/8/8/8/8 w - - 0 1",
            .expected = CastlingRights.None,
        },
        .{
            .fen = "8/8/8/8/8/8/8/8 w KQkq - 0 1",
            .expected = CastlingRights.All,
        },
        .{
            .fen = "8/8/8/8/8/8/8/8 w K - 0 1",
            .expected = CastlingRights.WhiteKingSide,
        },
        .{
            .fen = "8/8/8/8/8/8/8/8 w k - 0 1",
            .expected = CastlingRights.BlackKingSide,
        },
    }) |case| {
        const board = Board.fromFen(case.fen);
        try expectEqual(case.expected, board.getCastlingRights());
    }
}

test "fromFen enpassant" {
    const boardE3 = Board.fromFen("8/8/8/8/8/8/8/8 b - e3 0 1");
    try expectEqual(Square.E3, boardE3.getEnpassantSquare().?);
    
    const boardC6 = Board.fromFen("8/8/8/8/8/8/8/8 w - c6 0 1");
    try expectEqual(Square.C6, boardC6.getEnpassantSquare().?);
}

test "fromFen halfmove clock" {
    for ([_]FenTestCase(HalfmoveClock){
        .{ .fen = "8/8/8/8/8/8/8/8 w - - 0 1", .expected = 0 },
        .{ .fen = "8/8/8/8/8/8/8/8 w - - 123 1", .expected = 123 },
        .{ .fen = "8/8/8/8/8/8/8/8 w - - 999 1", .expected = 999 },
        .{ .fen = "8/8/8/8/8/8/8/8 w - - 100 1", .expected = 100 },
    }) |case| {
        const board = Board.fromFen(case.fen);
        try expectEqual(case.expected, board.getHalfmoveClock());
    }
}

test "fromFen fullmove counter" {
    for ([_]FenTestCase(FullmoveCounter){
        .{ .fen = "8/8/8/8/8/8/8/8 w - - 0 0", .expected = 1 },
        .{ .fen = "8/8/8/8/8/8/8/8 w - - 0 567", .expected = 1 },
        .{ .fen = "8/8/8/8/8/8/8/8 w - - 0 9999", .expected = 1 },
        .{ .fen = "8/8/8/8/8/8/8/8 w - - 0 4200", .expected = 1 },
    }) |case| {
        const board = Board.fromFen(case.fen);
        try expectEqual(case.expected, board.getFullmoveCounter());
    }
}