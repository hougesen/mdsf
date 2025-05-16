//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("write-good"),
    CommandType::Direct("write-good"),
    CommandType::Npm("write-good"),
    CommandType::Pnpm("write-good"),
    CommandType::Bun("write-good"),
    CommandType::Deno("write-good"),
    CommandType::Yarn("write-good"),
];

pub const IS_STDIN: bool = false;
