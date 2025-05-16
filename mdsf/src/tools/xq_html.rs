//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--html");
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("xq")];

pub const IS_STDIN: bool = true;
