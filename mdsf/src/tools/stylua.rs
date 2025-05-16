//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--verify");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("stylua"),
    CommandType::Direct("stylua"),
    CommandType::Npm("@johnnymorganz/stylua-bin"),
    CommandType::Pnpm("@johnnymorganz/stylua-bin"),
    CommandType::Bun("@johnnymorganz/stylua-bin"),
    CommandType::Deno("@johnnymorganz/stylua-bin"),
    CommandType::Yarn("@johnnymorganz/stylua-bin"),
];

pub const IS_STDIN: bool = false;
