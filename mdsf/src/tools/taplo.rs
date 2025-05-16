///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("taplo"),
    CommandType::Direct("taplo"),
    CommandType::Npm("@taplo/cli"),
    CommandType::Pnpm("@taplo/cli"),
    CommandType::Bun("@taplo/cli"),
    CommandType::Deno("@taplo/cli"),
    CommandType::Yarn("@taplo/cli"),
];

pub const IS_STDIN: bool = false;
