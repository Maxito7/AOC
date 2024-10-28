const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

pub fn main() !void {
    print("Hello, {s}!\n", .{"World"});
    const constant: u32 = 0;
    _ = constant;
    const constant2: u32 = 0;
    _ = constant2;
    const arr = [_]u32{};
    _ = arr;
}
