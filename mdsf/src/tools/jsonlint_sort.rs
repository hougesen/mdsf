//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-s");
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("jsonlint"),
    CommandType::Direct("jsonlint"),
    CommandType::Npm("jsonlint"),
    CommandType::Pnpm("jsonlint"),
    CommandType::Bun("jsonlint"),
    CommandType::Deno("jsonlint"),
    CommandType::Yarn("jsonlint"),
];

pub const IS_STDIN: bool = false;
