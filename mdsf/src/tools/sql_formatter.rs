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
    CommandType::NodeModules("sql-formatter"),
    CommandType::Direct("sql-formatter"),
    CommandType::Npm("sql-formatter", "sql-formatter"),
    CommandType::Pnpm("sql-formatter", "sql-formatter"),
    CommandType::Bun("sql-formatter", "sql-formatter"),
    CommandType::Deno("sql-formatter", "sql-formatter"),
    CommandType::Yarn("sql-formatter", "sql-formatter"),
];

pub const IS_STDIN: bool = false;
