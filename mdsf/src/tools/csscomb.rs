//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-t");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("csscomb"),
    CommandType::Direct("csscomb"),
    CommandType::Npm("csscomb", "csscomb"),
    CommandType::Pnpm("csscomb", "csscomb"),
    CommandType::Bun("csscomb", "csscomb"),
    CommandType::Deno("csscomb", "csscomb"),
    CommandType::Yarn("csscomb", "csscomb"),
];

pub const IS_STDIN: bool = false;
