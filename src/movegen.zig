const std = @import("std");
const assert = std.debug.assert;
const abs = std.math.absInt;

const Board = @import("board.zig").Board;
const Color = @import("color.zig").Color;
const Piece = @import("piece.zig").Piece;
const Square = @import("square.zig").Square;
const StaticBuffer = @import("buffer.zig").StaticBuffer;
const Move = @import("move.zig").Move;

pub const MoveGen = struct {
    const Self = @This();

    fn generatePawn(
        comptime Buffer: type,
        board: *const Board,
        output: *Buffer,
        from: Square,
    ) void {
        _ = board;

        output.add(Move.pawnSingle(from, from.forward(Color.White, 1)));
        output.add(Move.pawnDouble(from, from.forward(Color.White, 2)));
    }

    fn generateKnight(
        comptime Buffer: type,
        board: *const Board,
        output: *Buffer,
        from: Square,
    ) void {
        const file = from.getFileIndex();
        const rank = from.getRankIndex();

        for ([_]i8{ -2, -1, 1, 2 }) |dx| {
            for ([_]i8{ -2, -1, 1, 2 }) |dy| {
                if ((abs(dx) catch unreachable) + (abs(dy) catch unreachable) != 3) continue;

                if (file + dx > 7) continue;
                if (file + dx < 0) continue;
                if (rank + dy > 7) continue;
                if (rank + dy < 0) continue;

                var to = Square.fromFileRankIndexes(
                    @intCast(u6, file + dx),
                    @intCast(u6, rank + dy),
                );

                if (board.getPiece(to).equals(Piece.None)) {
                    output.add(Move.quiet(from, to));
                }
            }
        }
    }

    pub fn generate(
        comptime Buffer: type,
        board: *const Board,
        output: *Buffer,
    ) void {
        for (board.squares) |piece, squareIndex| {
            const square = Square.fromIndex(@truncate(u6, squareIndex));
            if (!piece.getColor().equals(board.getSideToMove())) continue;

            switch (piece.getDignity()) {
                .pawn => Self.generatePawn(Buffer, board, output, square),
                .knight => Self.generateKnight(Buffer, board, output, square),
                else => {},
            }
        }
    }
};

const expect = std.testing.expect;
const expectEqual = std.testing.expectEqual;

fn testGenerate(fen: []const u8) StaticBuffer(Move, 1024) {
    const board = Board.fromFen(fen);
    var buffer = StaticBuffer(Move, 1024).new();
    MoveGen.generate(StaticBuffer(Move, 1024), &board, &buffer);

    return buffer;
}

test "generate emptyboard" {
    const moves = testGenerate("8/8/8/8/8/8/8/8 w - - 0 1");
    try expectEqual(moves.length(), 0);
}

test "generate startpos" {
    const moves = testGenerate("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    try expectEqual(moves.length(), 20);

    try expect(moves.contains(Move.pawnSingle(Square.A2, Square.A3)));
    try expect(moves.contains(Move.pawnSingle(Square.B2, Square.B3)));
    try expect(moves.contains(Move.pawnSingle(Square.C2, Square.C3)));
    try expect(moves.contains(Move.pawnSingle(Square.D2, Square.D3)));
    try expect(moves.contains(Move.pawnSingle(Square.E2, Square.E3)));
    try expect(moves.contains(Move.pawnSingle(Square.F2, Square.F3)));
    try expect(moves.contains(Move.pawnSingle(Square.G2, Square.G3)));
    try expect(moves.contains(Move.pawnSingle(Square.H2, Square.H3)));

    try expect(moves.contains(Move.pawnDouble(Square.A2, Square.A4)));
    try expect(moves.contains(Move.pawnDouble(Square.B2, Square.B4)));
    try expect(moves.contains(Move.pawnDouble(Square.C2, Square.C4)));
    try expect(moves.contains(Move.pawnDouble(Square.D2, Square.D4)));
    try expect(moves.contains(Move.pawnDouble(Square.E2, Square.E4)));
    try expect(moves.contains(Move.pawnDouble(Square.F2, Square.F4)));
    try expect(moves.contains(Move.pawnDouble(Square.G2, Square.G4)));
    try expect(moves.contains(Move.pawnDouble(Square.H2, Square.H4)));

    try expect(moves.contains(Move.quiet(Square.B1, Square.A3)));
    try expect(moves.contains(Move.quiet(Square.B1, Square.C3)));
    try expect(moves.contains(Move.quiet(Square.G1, Square.F3)));
    try expect(moves.contains(Move.quiet(Square.G1, Square.H3)));
}
