///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("biome"),
    CommandType::Direct("biome"),
    CommandType::Npm("@biomejs/biome"),
    CommandType::Pnpm("@biomejs/biome"),
    CommandType::Bun("@biomejs/biome"),
    CommandType::Deno("@biomejs/biome"),
    CommandType::Yarn("@biomejs/biome"),
];

pub const IS_STDIN: bool = false;
