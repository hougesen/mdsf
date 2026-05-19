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
    cmd.arg("--output");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("openapi-format"),
    CommandType::Direct("openapi-format"),
    CommandType::Npm("openapi-format", "openapi-format"),
    CommandType::Pnpm("openapi-format", "openapi-format"),
    CommandType::Bun("openapi-format", "openapi-format"),
    CommandType::Deno("openapi-format", "openapi-format"),
    CommandType::Yarn("openapi-format", "openapi-format"),
];

pub const IS_STDIN: bool = false;
