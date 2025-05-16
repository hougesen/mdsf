//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-m");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("bibtex-tidy"),
    CommandType::Direct("bibtex-tidy"),
    CommandType::Npm("bibtex-tidy"),
    CommandType::Pnpm("bibtex-tidy"),
    CommandType::Bun("bibtex-tidy"),
    CommandType::Deno("bibtex-tidy"),
    CommandType::Yarn("bibtex-tidy"),
];

pub const IS_STDIN: bool = false;
