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
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 9] = [
    CommandType::NodeModules("fatou"),
    CommandType::Direct("fatou"),
    CommandType::Npm("fatou", "fatou"),
    CommandType::Pnpm("fatou", "fatou"),
    CommandType::Bun("fatou", "fatou"),
    CommandType::Deno("fatou", "fatou"),
    CommandType::Yarn("fatou", "fatou"),
    CommandType::Uv("fatou", "fatou"),
    CommandType::Pipx("fatou", "fatou"),
];

pub const IS_STDIN: bool = false;
