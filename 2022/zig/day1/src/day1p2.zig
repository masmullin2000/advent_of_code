const std = @import("std");
const lib = @import("lib.zig");
const stdout = std.io.getStdOut().writer();

pub fn main() !void {
    var list = try lib.get_elf_calories("input");

    if (list.items.len >= 3) {
        const rc = list.items[0] + list.items[1] + list.items[2];
        try stdout.print("{d}", .{rc});
    }

    list.deinit();
}

