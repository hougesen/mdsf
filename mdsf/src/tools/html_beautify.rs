//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-r");
    cmd.arg("--type");
    cmd.arg("html");
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("html-beautify"),
    CommandType::Direct("html-beautify"),
    CommandType::Npm("js-beautify", "js-beautify"),
    CommandType::Pnpm("js-beautify", "js-beautify"),
    CommandType::Bun("js-beautify", "js-beautify"),
    CommandType::Deno("js-beautify", "js-beautify"),
    CommandType::Yarn("js-beautify", "js-beautify"),
];

pub const IS_STDIN: bool = false;
