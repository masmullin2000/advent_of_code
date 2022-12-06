const std = @import("std");
const utils = @import("utils");
const lib = @import("lib.zig");

const stdout = std.io.getStdOut().writer();

pub fn main() !void {
    var lines = try utils.get_lines("input");
    var rc: u64 = 0;
    for (lines.items) |line| {
        rc += get_score_line(line);
    }

    try stdout.print("Total: {d}\n", .{rc});
}

fn get_score_line(line: []const u8) u8 {
    const theirs = lib.RPSChoice.new(line[0]);
    const outcome = lib.DesiredOutcome.new(line[2]);
    const mine = outcome.get_choice(theirs);

    return mine.value() + mine.outcome(theirs);
}
