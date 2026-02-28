//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-w");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("hongdown"),
    CommandType::Direct("hongdown"),
    CommandType::Npm("hongdown", "hongdown"),
    CommandType::Pnpm("hongdown", "hongdown"),
    CommandType::Bun("hongdown", "hongdown"),
    CommandType::Deno("hongdown", "hongdown"),
    CommandType::Yarn("hongdown", "hongdown"),
];

pub const IS_STDIN: bool = false;
