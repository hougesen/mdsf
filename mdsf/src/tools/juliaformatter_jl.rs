//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-E");
    cmd.arg(format!(
        "using JuliaFormatter;format_file(\"{}\")",
        file_path.to_string_lossy()
    ));
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("julia")];

pub const IS_STDIN: bool = false;
