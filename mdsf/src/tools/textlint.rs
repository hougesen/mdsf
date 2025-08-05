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
    CommandType::NodeModules("textlint"),
    CommandType::Direct("textlint"),
    CommandType::Npm("textlint", "textlint"),
    CommandType::Pnpm("textlint", "textlint"),
    CommandType::Bun("textlint", "textlint"),
    CommandType::Deno("textlint", "textlint"),
    CommandType::Yarn("textlint", "textlint"),
];

pub const IS_STDIN: bool = false;
