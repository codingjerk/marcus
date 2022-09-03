const std = @import("std");
const assert = std.debug.assert;

pub fn StaticBuffer(comptime E: type, comptime size: usize) type {
    return struct {
        buffer: [size]E,
        cursor: usize,

        const Self = @This();

        pub inline fn new() Self {
            return .{
                .buffer = undefined,
                .cursor = 0,
            };
        }

        pub inline fn length(self: *const Self) usize {
            return self.cursor;
        }

        pub inline fn get(self: *const Self, index: usize) E {
            assert(index < self.cursor);

            return self.buffer[index];
        }

        pub inline fn add(self: *Self, value: E) void {
            assert(self.cursor < size);

            self.buffer[self.cursor] = value;
            self.cursor += 1;
        }

        pub inline fn contains(self: *const Self, expected: E) bool {
            var i: usize = 0;
            while (i < self.cursor) : (i += 1) {
                const value = self.buffer[i];

                if (std.meta.eql(value, expected)) {
                    return true;
                }
            }

            return false;
        }
    };
}

const expect = std.testing.expect;
const expectEqual = std.testing.expectEqual;

test "basic usage" {
    var buffer = StaticBuffer(u8, 1024).new();
    try expectEqual(buffer.length(), 0);

    buffer.add(99);
    try expectEqual(buffer.length(), 1);
    try expectEqual(buffer.get(0), 99);

    buffer.add(42);
    try expectEqual(buffer.length(), 2);
    try expectEqual(buffer.get(0), 99);
    try expectEqual(buffer.get(1), 42);

    try expect(buffer.contains(99));
    try expect(buffer.contains(42));
}