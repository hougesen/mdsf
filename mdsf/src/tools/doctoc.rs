///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
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
    CommandType::NodeModules("doctoc"),
    CommandType::Direct("doctoc"),
    CommandType::Npm("doctoc"),
    CommandType::Pnpm("doctoc"),
    CommandType::Bun("doctoc"),
    CommandType::Deno("doctoc"),
    CommandType::Yarn("doctoc"),
];

pub const IS_STDIN: bool = false;
