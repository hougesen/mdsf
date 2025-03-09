///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
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
    CommandType::NodeModules("autocorrect"),
    CommandType::Direct("autocorrect"),
    CommandType::Npm("autocorrect-node"),
    CommandType::Pnpm("autocorrect-node"),
    CommandType::Bun("autocorrect-node"),
    CommandType::Deno("autocorrect-node"),
    CommandType::Yarn("autocorrect-node"),
];

pub const IS_STDIN: bool = false;
