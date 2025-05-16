///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("alex"),
    CommandType::Direct("alex"),
    CommandType::Npm("alex"),
    CommandType::Pnpm("alex"),
    CommandType::Bun("alex"),
    CommandType::Deno("alex"),
    CommandType::Yarn("alex"),
];

pub const IS_STDIN: bool = false;
