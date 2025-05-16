///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg("--stdin");
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("standard"),
    CommandType::Direct("standard"),
    CommandType::Npm("standard"),
    CommandType::Pnpm("standard"),
    CommandType::Bun("standard"),
    CommandType::Deno("standard"),
    CommandType::Yarn("standard"),
];

pub const IS_STDIN: bool = true;
