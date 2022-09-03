// Square is coordinates of board square
// starting from bottom-left (A1) as 0
// and moving to the right (H1) and
// then to top (A2, then, A3), until H8
// It's 6 bits for 64 possible states
// Without "null" square
pub const Square = packed struct {
    index: u6,

    const Self = @This();
    
    pub inline fn fromFen(file: u8, rank: u8) Self {
        var index = (file - 'a') + (rank - '1') * 8;
        if (index > Self.Max.index) unreachable;

        return Self.fromIndex(@truncate(u6, index));
    }

    pub inline fn fromIndex(index: u6) Self {
        return .{ .index = index };
    }

    pub inline fn getIndex(self: Self) u6 {
        return self.index;
    }

    pub inline fn equals(self: Self, other: Self) bool {
        return self.index == other.index;
    }

    pub inline fn moveRightUnchecked(self: *Self, amount: u6) void {
        self.index +%= amount;
    }

    pub inline fn moveDownUnchecked(self: *Self, amount: u6) void {
        self.index -%= amount * 8;
    }
    
    pub const None = Self { .index = 0b0000 };

    pub const A1 = Self { .index = 0 };
    pub const B1 = Self { .index = 1 };
    pub const C1 = Self { .index = 2 };
    pub const D1 = Self { .index = 3 };
    pub const E1 = Self { .index = 4 };
    pub const F1 = Self { .index = 5 };
    pub const G1 = Self { .index = 6 };
    pub const H1 = Self { .index = 7 };

    pub const A2 = Self { .index = 8 };
    pub const B2 = Self { .index = 9 };
    pub const C2 = Self { .index = 10 };
    pub const D2 = Self { .index = 11 };
    pub const E2 = Self { .index = 12 };
    pub const F2 = Self { .index = 13 };
    pub const G2 = Self { .index = 14 };
    pub const H2 = Self { .index = 15 };

    pub const A3 = Self { .index = 16 };
    pub const B3 = Self { .index = 17 };
    pub const C3 = Self { .index = 18 };
    pub const D3 = Self { .index = 19 };
    pub const E3 = Self { .index = 20 };
    pub const F3 = Self { .index = 21 };
    pub const G3 = Self { .index = 22 };
    pub const H3 = Self { .index = 23 };

    pub const A4 = Self { .index = 24 };
    pub const B4 = Self { .index = 25 };
    pub const C4 = Self { .index = 26 };
    pub const D4 = Self { .index = 27 };
    pub const E4 = Self { .index = 28 };
    pub const F4 = Self { .index = 29 };
    pub const G4 = Self { .index = 30 };
    pub const H4 = Self { .index = 31 };

    pub const A6 = Self { .index = 40 };
    pub const B6 = Self { .index = 41 };
    pub const C6 = Self { .index = 42 };
    pub const D6 = Self { .index = 43 };
    pub const E6 = Self { .index = 44 };
    pub const F6 = Self { .index = 45 };
    pub const G6 = Self { .index = 46 };
    pub const H6 = Self { .index = 47 };

    pub const A7 = Self { .index = 48 };
    pub const B7 = Self { .index = 49 };
    pub const C7 = Self { .index = 50 };
    pub const D7 = Self { .index = 51 };
    pub const E7 = Self { .index = 52 };
    pub const F7 = Self { .index = 53 };
    pub const G7 = Self { .index = 54 };
    pub const H7 = Self { .index = 55 };
    
    pub const A8 = Self { .index = 56 };
    pub const B8 = Self { .index = 57 };
    pub const C8 = Self { .index = 58 };
    pub const D8 = Self { .index = 59 };
    pub const E8 = Self { .index = 60 };
    pub const F8 = Self { .index = 61 };
    pub const G8 = Self { .index = 62 };
    pub const H8 = Self { .index = 63 };

    pub const Max = H8;
};