//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub const fn set_args(
    cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("jq")];

pub const IS_STDIN: bool = true;
