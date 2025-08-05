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

pub const COMMANDS: [CommandType; 8] = [
    CommandType::NodeModules("tsqllint"),
    CommandType::Dotnet("tsqllint"),
    CommandType::Direct("tsqllint"),
    CommandType::Npm("tsqllint", "tsqllint"),
    CommandType::Pnpm("tsqllint", "tsqllint"),
    CommandType::Bun("tsqllint", "tsqllint"),
    CommandType::Deno("tsqllint", "tsqllint"),
    CommandType::Yarn("tsqllint", "tsqllint"),
];

pub const IS_STDIN: bool = false;
