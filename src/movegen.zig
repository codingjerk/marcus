const std = @import("std");
const assert = std.debug.assert;

const Board = @import("board.zig").Board;
const Piece = @import("piece.zig").Piece;
const Square = @import("square.zig").Square;
const StaticBuffer = @import("buffer.zig").StaticBuffer;
const Move = @import("move.zig").Move;

pub const MoveGen = struct {
    pub fn generate(
        comptime Output: type,
        board: *const Board,
        output: *Output,
    ) void {
        for (board.squares) |piece, squareIndex| {
            std.log.warn("{}, {}", .{ piece, squareIndex });            
        }

        if (board.getPiece(Square.A2).equals(Piece.WhitePawn)) {
            output.add(Move.pawnSingle(Square.A2, Square.A3));
            output.add(Move.pawnSingle(Square.B2, Square.B3));
            output.add(Move.pawnSingle(Square.C2, Square.C3));
            output.add(Move.pawnSingle(Square.D2, Square.D3));
            output.add(Move.pawnSingle(Square.E2, Square.E3));
            output.add(Move.pawnSingle(Square.F2, Square.F3));
            output.add(Move.pawnSingle(Square.G2, Square.G3));
            output.add(Move.pawnSingle(Square.H2, Square.H3));

            output.add(Move.pawnSingle(Square.A2, Square.A4));
            output.add(Move.pawnSingle(Square.B2, Square.B4));
            output.add(Move.pawnSingle(Square.C2, Square.C4));
            output.add(Move.pawnSingle(Square.D2, Square.D4));
            output.add(Move.pawnSingle(Square.E2, Square.E4));
            output.add(Move.pawnSingle(Square.F2, Square.F4));
            output.add(Move.pawnSingle(Square.G2, Square.G4));
            output.add(Move.pawnSingle(Square.H2, Square.H4));

            output.add(Move.pawnSingle(Square.B1, Square.A3));
            output.add(Move.pawnSingle(Square.B1, Square.C3));
            output.add(Move.pawnSingle(Square.G1, Square.F3));
            output.add(Move.pawnSingle(Square.G1, Square.H3));
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