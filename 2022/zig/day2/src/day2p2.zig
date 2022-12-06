const std = @import("std");
const utils = @import("utils");
const lib = @import("lib.zig");
const stdout = std.io.getStdOut().writer();

pub fn doit() !void {
    var buffer: [524288]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&buffer);
    
    //var arena = std.heap.ArenaAllocator.init(std.heap.c_allocator);
    var arena = std.heap.ArenaAllocator.init(fba.allocator());
    const alloc = arena.allocator();
    defer _ = arena.deinit();
    //errdefer _ = arena.deinit();

    var lines = try utils.get_lines("input", alloc);

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

    return lib.get_score(mine, theirs);
}

pub fn main() !void {
    var i: u32 = 0;
    while (i < 2500) : (i += 1) {
        try doit();
    }
}
