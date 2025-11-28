//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("oxfmt"),
    CommandType::Direct("oxfmt"),
    CommandType::Npm("oxfmt", "oxfmt"),
    CommandType::Pnpm("oxfmt", "oxfmt"),
    CommandType::Bun("oxfmt", "oxfmt"),
    CommandType::Deno("oxfmt", "oxfmt"),
    CommandType::Yarn("oxfmt", "oxfmt"),
];

pub const IS_STDIN: bool = false;
