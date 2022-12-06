const std = @import("std");

const lib = @import("lib.zig");

//const ArrayList = std.ArrayList;
const stdout = std.io.getStdOut().writer();
//var gpa = std.heap.GeneralPurposeAllocator(.{}){};
//const allocator = gpa.allocator();

pub fn main() !void {
    var list = try lib.get_elf_calories("input");

    if (list.items.len > 0) {
        try stdout.print("{d}", .{list.items[0]});
    }

    list.deinit();
}

