//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--format");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("npm-groovy-lint"),
    CommandType::Direct("npm-groovy-lint"),
    CommandType::Npm("npm-groovy-lint"),
    CommandType::Pnpm("npm-groovy-lint"),
    CommandType::Bun("npm-groovy-lint"),
    CommandType::Deno("npm-groovy-lint"),
    CommandType::Yarn("npm-groovy-lint"),
];

pub const IS_STDIN: bool = false;
