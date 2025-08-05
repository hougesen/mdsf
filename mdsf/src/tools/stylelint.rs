//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("stylelint"),
    CommandType::Direct("stylelint"),
    CommandType::Npm("stylelint", "stylelint"),
    CommandType::Pnpm("stylelint", "stylelint"),
    CommandType::Bun("stylelint", "stylelint"),
    CommandType::Deno("stylelint", "stylelint"),
    CommandType::Yarn("stylelint", "stylelint"),
];

pub const IS_STDIN: bool = false;
