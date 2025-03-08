///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 6] = [
    CommandType::NodeModules("htmlhint"),
    CommandType::Direct("htmlhint"),
    CommandType::Npm("htmlhint"),
    CommandType::Pnpm("htmlhint"),
    CommandType::Bun("htmlhint"),
    CommandType::Deno("htmlhint"),
];

pub const IS_STDIN: bool = false;
