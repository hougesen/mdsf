//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--embedded-language-formatting");
    cmd.arg("off");
    cmd.arg("--log-level");
    cmd.arg("error");
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("prettier"),
    CommandType::Direct("prettier"),
    CommandType::Npm("prettier"),
    CommandType::Pnpm("prettier"),
    CommandType::Bun("prettier"),
    CommandType::Deno("prettier"),
    CommandType::Yarn("prettier"),
];

pub const IS_STDIN: bool = false;
