//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("check");
    cmd.arg("--quiet");
    cmd.arg("--no-respect-gitignore");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("fortitude"),
    CommandType::Uv("fortitude-lint", "fortitude"),
    CommandType::Pipx("fortitude-lint", "fortitude"),
];

pub const IS_STDIN: bool = false;
