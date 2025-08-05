//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("buf"),
    CommandType::Direct("buf"),
    CommandType::Npm("@bufbuild/buf", "@bufbuild/buf"),
    CommandType::Pnpm("@bufbuild/buf", "@bufbuild/buf"),
    CommandType::Bun("@bufbuild/buf", "@bufbuild/buf"),
    CommandType::Deno("@bufbuild/buf", "@bufbuild/buf"),
    CommandType::Yarn("@bufbuild/buf", "@bufbuild/buf"),
];

pub const IS_STDIN: bool = false;
