// Right bits:
// 0000
//    ^ - QueenSide Black
//   ^ - KingSide Black
//  ^ - QueenSide White
// ^ - KingSide White
pub const Rights = packed struct {
    index: u4,

    const Self = @This();
    
    pub const None = Self { .index = 0b0000 };
    pub const All  = Self { .index = 0b1111 };

    pub const WhiteKingSide  = Self { .index = 0b1000 };
    pub const WhiteQueenSide = Self { .index = 0b0100 };
    pub const BlackKingSide  = Self { .index = 0b0010 };
    pub const BlackQueenSide = Self { .index = 0b0001 };

    pub inline fn unset(self: *Self) void {
        self.index = None.index;
    }

    pub inline fn setFromFen(self: *Self, fen: u8) void {
        self.index |= switch (fen) {
            'K' => WhiteKingSide.index,
            'Q' => WhiteQueenSide.index,
            'k' => BlackKingSide.index,
            'q' => BlackQueenSide.index,

            else => unreachable,
        };
    }
};