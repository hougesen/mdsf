//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format-in-place");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("purs-tidy"),
    CommandType::Direct("purs-tidy"),
    CommandType::Npm("purs-tidy", "purs-tidy"),
    CommandType::Pnpm("purs-tidy", "purs-tidy"),
    CommandType::Bun("purs-tidy", "purs-tidy"),
    CommandType::Deno("purs-tidy", "purs-tidy"),
    CommandType::Yarn("purs-tidy", "purs-tidy"),
];

pub const IS_STDIN: bool = false;
