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
    cmd.arg(format!("--schema={}", file_path.to_string_lossy()));
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("prisma"),
    CommandType::Direct("prisma"),
    CommandType::Npm("prisma", "prisma"),
    CommandType::Pnpm("prisma", "prisma"),
    CommandType::Bun("prisma", "prisma"),
    CommandType::Deno("prisma", "prisma"),
    CommandType::Yarn("prisma", "prisma"),
];

pub const IS_STDIN: bool = false;
