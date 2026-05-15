const std = @import("std");

const ProjectMap = struct {
    alias: []const u8,
    path: []const u8,
};

pub fn main(init: std.process.Init) !void {
    const io = init.io;

    var stdout_writer = std.Io.File.stdout().writer(io, &.{});
    var stderr_writer = std.Io.File.stderr().writer(io, &.{});
    const stdout = &stdout_writer.interface;
    const stderr = &stderr_writer.interface;

    const args_iter = try init.minimal.args.toSlice(init.arena.allocator());
    if (args_iter.len < 2) {
        try stderr.print("Error: No Project Alias provided. \nUsage: zsm-bin <alias>\n", .{});
        std.process.exit(1);
    }

    const target_alias = args_iter[1];

    const projects = [_]ProjectMap{
        .{ .alias = "roguelike", .path = "/home/bishesh/dev/entropy-descent" },
        .{ .alias = "whale", .path = "/home/bishesh/dev/WhaleWatchers" },
        .{ .alias = "config", .path = "/home/bishesh/.config/nvim" },
    };

    for (projects) |project| {
        if (std.mem.eql(u8, project.alias, target_alias)) {
            try stdout.print("{s}", .{project.path});
            return;
        }
    }
    try stderr.print("Error: Project alias '{s}' not found in registry.\n", .{target_alias});
    std.process.exit(1);
}
