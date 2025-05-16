//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--elm-version=0.19");
    cmd.arg("--yes");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("elm-format"),
    CommandType::Direct("elm-format"),
    CommandType::Npm("elm-format"),
    CommandType::Pnpm("elm-format"),
    CommandType::Bun("elm-format"),
    CommandType::Deno("elm-format"),
    CommandType::Yarn("elm-format"),
];

pub const IS_STDIN: bool = false;
