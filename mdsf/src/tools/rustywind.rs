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
    CommandType::NodeModules("rustywind"),
    CommandType::Direct("rustywind"),
    CommandType::Npm("rustywind", "rustywind"),
    CommandType::Pnpm("rustywind", "rustywind"),
    CommandType::Bun("rustywind", "rustywind"),
    CommandType::Deno("rustywind", "rustywind"),
    CommandType::Yarn("rustywind", "rustywind"),
];

pub const IS_STDIN: bool = false;
