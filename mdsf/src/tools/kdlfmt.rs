//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("kdlfmt"),
    CommandType::Direct("kdlfmt"),
    CommandType::Npm("kdlfmt", "kdlfmt"),
    CommandType::Pnpm("kdlfmt", "kdlfmt"),
    CommandType::Bun("kdlfmt", "kdlfmt"),
    CommandType::Deno("kdlfmt", "kdlfmt"),
    CommandType::Yarn("kdlfmt", "kdlfmt"),
];

pub const IS_STDIN: bool = false;
