//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--in-place");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("autopep8"),
    CommandType::Uv("autopep8", "autopep8"),
    CommandType::Pipx("autopep8", "autopep8"),
];

pub const IS_STDIN: bool = false;
