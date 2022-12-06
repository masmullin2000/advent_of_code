const std = @import("std");

const stdout = std.io.getStdOut().writer();
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

const ArrayList = std.ArrayList;

pub fn get_lines(input: []const u8) !ArrayList([]u8) {
    var file = try std.fs.cwd().openFile(input, .{});
    defer file.close();

    var buf = std.io.bufferedReader(file.reader());
    var stream = buf.reader();

    var list = ArrayList([]u8).init(allocator);

    var data: [4096]u8 = undefined;
    while (try stream.readUntilDelimiterOrEof(&data, '\n')) |line| {
        var d = try allocator.alloc(u8, line.len);
        std.mem.copy(u8, d, line[0..line.len]);

        try list.append(d);
    }

    return list;
}
