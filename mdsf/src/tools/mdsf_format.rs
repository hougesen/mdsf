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
    CommandType::NodeModules("mdsf"),
    CommandType::Direct("mdsf"),
    CommandType::Npm("mdsf-cli", "mdsf-cli"),
    CommandType::Pnpm("mdsf-cli", "mdsf-cli"),
    CommandType::Bun("mdsf-cli", "mdsf-cli"),
    CommandType::Deno("mdsf-cli", "mdsf-cli"),
    CommandType::Yarn("mdsf-cli", "mdsf-cli"),
];

pub const IS_STDIN: bool = false;
