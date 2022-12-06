const std = @import("std");
const utils = @import("utils");
const lib = @import("lib.zig");

const stdout = std.io.getStdOut().writer();

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

pub fn main() !void {
    var lines = try utils.get_lines("input", allocator);
    defer _ = lines.deinit();

    var rc: u64 = 0;
    for (lines.items) |line| {
        rc += get_score_line(line);
        allocator.free(line);
    }

    try stdout.print("Total: {d}\n", .{rc});
}

fn get_score_line(line: []const u8) u8 {
    const theirs = lib.RPSChoice.new(line[0]);
    const mine = lib.RPSChoice.new(line[2]);

    return lib.get_score(mine, theirs);
}
