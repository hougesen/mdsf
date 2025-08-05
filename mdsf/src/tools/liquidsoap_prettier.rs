//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("liquidsoap-prettier"),
    CommandType::Direct("liquidsoap-prettier"),
    CommandType::Npm("liquidsoap-prettier", "liquidsoap-prettier"),
    CommandType::Pnpm("liquidsoap-prettier", "liquidsoap-prettier"),
    CommandType::Bun("liquidsoap-prettier", "liquidsoap-prettier"),
    CommandType::Deno("liquidsoap-prettier", "liquidsoap-prettier"),
    CommandType::Yarn("liquidsoap-prettier", "liquidsoap-prettier"),
];

pub const IS_STDIN: bool = false;
