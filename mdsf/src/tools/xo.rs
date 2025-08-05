//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg("--stdin");
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("xo"),
    CommandType::Direct("xo"),
    CommandType::Npm("xo", "xo"),
    CommandType::Pnpm("xo", "xo"),
    CommandType::Bun("xo", "xo"),
    CommandType::Deno("xo", "xo"),
    CommandType::Yarn("xo", "xo"),
];

pub const IS_STDIN: bool = true;
