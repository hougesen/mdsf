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
    CommandType::Npm("textlint"),
    CommandType::Pnpm("textlint"),
    CommandType::Bun("textlint"),
    CommandType::Deno("textlint"),
    CommandType::Yarn("textlint"),
];

pub const IS_STDIN: bool = false;
