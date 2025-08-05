//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("markuplint"),
    CommandType::Direct("markuplint"),
    CommandType::Npm("markuplint", "markuplint"),
    CommandType::Pnpm("markuplint", "markuplint"),
    CommandType::Bun("markuplint", "markuplint"),
    CommandType::Deno("markuplint", "markuplint"),
    CommandType::Yarn("markuplint", "markuplint"),
];

pub const IS_STDIN: bool = false;
