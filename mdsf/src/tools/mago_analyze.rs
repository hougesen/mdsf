//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("analyze");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 2] =
    [CommandType::PhpVendor("mago"), CommandType::Direct("mago")];

pub const IS_STDIN: bool = false;
