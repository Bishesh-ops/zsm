const std = @import("std");

pub fn main(init: std.process.Init) !void {
    const io = init.io;

    var stdout_writer = std.Io.File.stdout().writer(io, &.{});
    var stderr_writer = std.Io.File.stderr().writer(io, &.{});
    const stdout = &stdout_writer.interface;
    const stderr = &stderr_writer.interface;

    const args_iter = try init.minimal.args.toSlice(init.arena.allocator());
    if (args_iter.len < 2) {
        try stderr.print("Error: No search alias provided. \nUsage: zsm <alias>\n", .{});
        std.process.exit(1);
    }
    const target_alias = args_iter[1];

    const base_directory_path = "/home/bisheshshrestha/Dev";

    var dir = std.Io.Dir.openDirAbsolute(io, base_directory_path, .{ .iterate = true }) catch |err| {
        try stderr.print("Error could not open Dev directory. {any}\n", .{err});
        std.process.exit(1);
    };
    defer dir.close(io);

    var walker = dir.iterate();

    while (try walker.next(io)) |entry| {
        if (entry.kind == .directory) {
            if (std.ascii.indexOfIgnoreCase(entry.name, target_alias) != null) {
                try stdout.print("{s}/{s}", .{ base_directory_path, entry.name });
                return;
            }
        }
    }

    try stderr.print("Error: No project matching '{s}' found in '{s}'.\n", .{ target_alias, base_directory_path });
    std.process.exit(1);
}
