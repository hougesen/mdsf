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
    CommandType::NodeModules("quick-lint-js"),
    CommandType::Direct("quick-lint-js"),
    CommandType::Npm("quick-lint-js"),
    CommandType::Pnpm("quick-lint-js"),
    CommandType::Bun("quick-lint-js"),
    CommandType::Deno("quick-lint-js"),
    CommandType::Yarn("quick-lint-js"),
];

pub const IS_STDIN: bool = false;
