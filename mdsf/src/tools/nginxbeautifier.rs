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
    CommandType::NodeModules("nginxbeautifier"),
    CommandType::Direct("nginxbeautifier"),
    CommandType::Npm("nginxbeautifier"),
    CommandType::Pnpm("nginxbeautifier"),
    CommandType::Bun("nginxbeautifier"),
    CommandType::Deno("nginxbeautifier"),
    CommandType::Yarn("nginxbeautifier"),
];

pub const IS_STDIN: bool = false;
