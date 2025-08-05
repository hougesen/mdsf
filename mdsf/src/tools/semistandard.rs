//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg("--stdin");
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("semistandard"),
    CommandType::Direct("semistandard"),
    CommandType::Npm("semistandard", "semistandard"),
    CommandType::Pnpm("semistandard", "semistandard"),
    CommandType::Bun("semistandard", "semistandard"),
    CommandType::Deno("semistandard", "semistandard"),
    CommandType::Yarn("semistandard", "semistandard"),
];

pub const IS_STDIN: bool = true;
