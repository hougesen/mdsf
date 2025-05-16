//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("markdownlint-cli2"),
    CommandType::Direct("markdownlint-cli2"),
    CommandType::Npm("markdownlint-cli2"),
    CommandType::Pnpm("markdownlint-cli2"),
    CommandType::Bun("markdownlint-cli2"),
    CommandType::Deno("markdownlint-cli2"),
    CommandType::Yarn("markdownlint-cli2"),
];

pub const IS_STDIN: bool = false;
