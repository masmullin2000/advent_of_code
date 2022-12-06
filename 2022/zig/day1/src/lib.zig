const std = @import("std");
const utils = @import("utils");

const stdout = std.io.getStdOut().writer();
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

const ArrayList = std.ArrayList;

pub fn get_elf_calories(input: []const u8) !ArrayList(u64) {
    var list = try utils.get_lines(input);
    defer list.deinit();

    var rc = ArrayList(u64).init(allocator);
    var curr: u64 = 0;
    for (list.items) |line| {
        if (line.len == 0) {
            try rc.append(curr);
            curr = 0;
        } else {
            var as_int = try std.fmt.parseInt(u64, line, 10);
            curr += as_int;
        }
    }
    try rc.append(curr);

    std.sort.sort(u64, rc.items, {}, comptime std.sort.desc(u64));

    return rc;
}
