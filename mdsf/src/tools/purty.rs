//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("purty"),
    CommandType::Direct("purty"),
    CommandType::Npm("purty"),
    CommandType::Pnpm("purty"),
    CommandType::Bun("purty"),
    CommandType::Deno("purty"),
    CommandType::Yarn("purty"),
];

pub const IS_STDIN: bool = false;
