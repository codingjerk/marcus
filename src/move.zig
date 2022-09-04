const std = @import("std");

const assert = std.debug.assert;
// TODO: usingnamespace @import("prelude");
const Piece = @import("piece.zig").Piece;
const Square = @import("square.zig").Square;

// Move bits
// TODO: from, to, special, capture, moved, promotion
// PERF: try to move more stuff to undo table
pub const Move = packed struct {
    from: Square,
    to: Square,
    captured: Piece,

    const Self = @This();

    pub inline fn capture(from: Square, to: Square, captured: Piece) Move {
        return Move {
            .from = from,
            .to = to,
            .captured = captured,
        };
    }

    pub inline fn quiet(from: Square, to: Square) Self {
        return capture(from, to, Piece.None);
    }

    pub inline fn pawnSingle(from: Square, to: Square) Self {
        return quiet(from, to);
    }

    pub inline fn pawnDouble(from: Square, to: Square) Self {
        return quiet(from, to);
    }

    pub inline fn pawnCapture(from: Square, to: Square, captured: Piece) Self {
        return capture(from, to, captured);
    }

    pub inline fn enPassant(from: Square, to: Square, captured: Piece) Self {
        return capture(from, to, captured);
    }
};

comptime {
    assert(@bitSizeOf(Move) == 6 + 6 + 4);
}

const expectEqual = std.testing.expectEqual;

test "compiler regression" {
    const move = Move.capture(Square.A2, Square.A3, Piece.None);

    try expectEqual(Square.A2, move.from);
    try expectEqual(Square.A3, move.to);
    try expectEqual(Piece.None, move.captured);
}
