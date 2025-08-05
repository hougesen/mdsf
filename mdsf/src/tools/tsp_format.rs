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
    CommandType::NodeModules("tsp"),
    CommandType::Direct("tsp"),
    CommandType::Npm("@typespec/compiler", "tsp"),
    CommandType::Pnpm("@typespec/compiler", "tsp"),
    CommandType::Bun("@typespec/compiler", "tsp"),
    CommandType::Deno("@typespec/compiler", "tsp"),
    CommandType::Yarn("@typespec/compiler", "tsp"),
];

pub const IS_STDIN: bool = false;
