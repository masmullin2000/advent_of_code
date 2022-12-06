const std = @import("std");

const stdout = std.io.getStdOut().writer();
const Allocator = std.mem.Allocator;

const ArrayList = std.ArrayList;

fn clean_array(list: ArrayList([]u8), allocator: Allocator) void {
    for (list.items) |it| {
        allocator.free(it);
    }
    list.deinit();
}

pub fn get_lines(input: []const u8, allocator: Allocator) !ArrayList([]u8) {
    var file = try std.fs.cwd().openFile(input, .{});
    defer file.close();

    var buf = std.io.bufferedReader(file.reader());
    var stream = buf.reader();

    var list = ArrayList([]u8).init(allocator);
    errdefer _ = clean_array(list, allocator); 

    var data: [4096]u8 = undefined;
    while (try stream.readUntilDelimiterOrEof(&data, '\n')) |line| {
        var d = try allocator.alloc(u8, line.len);
        std.mem.copy(u8, d, line[0..line.len]);

        try list.append(d);
    }

    return list;
}
