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

pub const COMMANDS: [CommandType; 9] = [
    CommandType::NodeModules("squawk"),
    CommandType::Direct("squawk"),
    CommandType::Npm("squawk-cli"),
    CommandType::Pnpm("squawk-cli"),
    CommandType::Bun("squawk-cli"),
    CommandType::Deno("squawk-cli"),
    CommandType::Yarn("squawk-cli"),
    CommandType::Uv("squawk-cli", "squawk-cli"),
    CommandType::Pipx("squawk-cli"),
];

pub const IS_STDIN: bool = false;
