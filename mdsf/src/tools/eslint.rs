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
    CommandType::NodeModules("eslint"),
    CommandType::Direct("eslint"),
    CommandType::Npm("eslint"),
    CommandType::Pnpm("eslint"),
    CommandType::Bun("eslint"),
    CommandType::Deno("eslint"),
    CommandType::Yarn("eslint"),
];

pub const IS_STDIN: bool = false;
