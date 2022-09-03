const Color = @import("color.zig").Color;

pub const Dignity = enum(u3) {
    none   = 0b000,
    pawn   = 0b001,
    knight = 0b010,
    bishop = 0b011,
    rook   = 0b100,
    queen  = 0b101,
    king   = 0b110,
    // One free value left
};

// Piece bits:
// 0000
//  ^-^ - 3 dignity bits
// ^ - color bit
pub const Piece = packed struct {
    index: u4,

    const Self = @This();
    
    pub const None        = Self { .index = 0b0000 };

    pub const BlackPawn   = Self { .index = 0b0001 };
    pub const BlackKnight = Self { .index = 0b0010 };
    pub const BlackBishop = Self { .index = 0b0011 };
    pub const BlackRook   = Self { .index = 0b0100 };
    pub const BlackQueen  = Self { .index = 0b0101 };
    pub const BlackKing   = Self { .index = 0b0110 };

    pub const WhitePawn   = Self { .index = 0b1001 };
    pub const WhiteKnight = Self { .index = 0b1010 };
    pub const WhiteBishop = Self { .index = 0b1011 };
    pub const WhiteRook   = Self { .index = 0b1100 };
    pub const WhiteQueen  = Self { .index = 0b1101 };
    pub const WhiteKing   = Self { .index = 0b1110 };
    
    pub inline fn fromFen(fen: u8) Self {
        return switch (fen) {
            'p' => BlackPawn,
            'n' => BlackKnight,
            'b' => BlackBishop,
            'r' => BlackRook,
            'q' => BlackQueen,
            'k' => BlackKing,

            'P' => WhitePawn,
            'N' => WhiteKnight,
            'B' => WhiteBishop,
            'R' => WhiteRook,
            'Q' => WhiteQueen,
            'K' => WhiteKing,

            else => unreachable,
        };
    }

    pub inline fn equals(self: Self, other: Self) bool {
        return self.index == other.index;
    }

    pub inline fn getColor(self: Self) Color {
        // PERF: try to make it better
        return switch (self.index & 0b1000) {
            0b0000 => Color.Black,
            0b1000 => Color.White,
            else => unreachable,
        };
    }

    pub inline fn getDignity(self: Self) Dignity {
        return @intToEnum(Dignity, @truncate(u3, self.index));
    }
};