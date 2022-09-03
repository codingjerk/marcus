// Color bits
pub const Color = packed struct {
    index: u1,

    const Self = @This();
    
    pub inline fn equals(self: Self, other: Self) bool {
        return self.index == other.index;
    }

    pub const Black = Self { .index = 0b0 };
    pub const White = Self { .index = 0b1 };
};