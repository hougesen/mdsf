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
    let fps = file_path.to_string_lossy();
    cmd.arg(format!("--schema={fps}"));
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("prisma"),
    CommandType::Direct("prisma"),
    CommandType::Npm("prisma"),
    CommandType::Pnpm("prisma"),
    CommandType::Bun("prisma"),
    CommandType::Deno("prisma"),
    CommandType::Yarn("prisma"),
];

pub const IS_STDIN: bool = false;
