//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("md-padding"),
    CommandType::Direct("md-padding"),
    CommandType::Npm("md-padding"),
    CommandType::Pnpm("md-padding"),
    CommandType::Bun("md-padding"),
    CommandType::Deno("md-padding"),
    CommandType::Yarn("md-padding"),
];

pub const IS_STDIN: bool = false;
