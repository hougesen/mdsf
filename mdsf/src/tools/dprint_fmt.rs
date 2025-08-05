//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fmt");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("dprint"),
    CommandType::Direct("dprint"),
    CommandType::Npm("dprint", "dprint"),
    CommandType::Pnpm("dprint", "dprint"),
    CommandType::Bun("dprint", "dprint"),
    CommandType::Deno("dprint", "dprint"),
    CommandType::Yarn("dprint", "dprint"),
];

pub const IS_STDIN: bool = false;
