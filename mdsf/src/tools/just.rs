//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fmt");
    cmd.arg("--unstable");
    cmd.arg("--justfile");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 9] = [
    CommandType::NodeModules("just"),
    CommandType::Direct("just"),
    CommandType::Npm("rust-just"),
    CommandType::Pnpm("rust-just"),
    CommandType::Bun("rust-just"),
    CommandType::Deno("rust-just"),
    CommandType::Yarn("rust-just"),
    CommandType::Uv("rust-just", "just"),
    CommandType::Pipx("rust-just"),
];

pub const IS_STDIN: bool = false;
