//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg("-");
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("tombi"),
    CommandType::Uv("tombi", "tombi"),
    CommandType::Pipx("tombi", "tombi"),
];

pub const IS_STDIN: bool = true;
