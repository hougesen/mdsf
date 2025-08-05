//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-w");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("fixjson"),
    CommandType::Direct("fixjson"),
    CommandType::Npm("fixjson", "fixjson"),
    CommandType::Pnpm("fixjson", "fixjson"),
    CommandType::Bun("fixjson", "fixjson"),
    CommandType::Deno("fixjson", "fixjson"),
    CommandType::Yarn("fixjson", "fixjson"),
];

pub const IS_STDIN: bool = false;
