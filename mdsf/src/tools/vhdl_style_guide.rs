//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd.arg("--fix");
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("vsg"),
    CommandType::Uv("vsg", "vsg"),
    CommandType::Pipx("vsg"),
];

pub const IS_STDIN: bool = false;
