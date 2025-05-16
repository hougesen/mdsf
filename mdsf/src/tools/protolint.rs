//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("lint");
    cmd.arg("-fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("protolint"),
    CommandType::Direct("protolint"),
    CommandType::Npm("protolint"),
    CommandType::Pnpm("protolint"),
    CommandType::Bun("protolint"),
    CommandType::Deno("protolint"),
    CommandType::Yarn("protolint"),
];

pub const IS_STDIN: bool = false;
