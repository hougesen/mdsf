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
    cmd.arg("--nobackup");
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("oelint-adv"),
    CommandType::Uv("oelint_adv", "oelint-adv"),
    CommandType::Pipx("oelint_adv", "oelint-adv"),
];

pub const IS_STDIN: bool = false;
