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
    cmd.arg("--fix");
    cmd.arg("--noPrompt");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("solhint"),
    CommandType::Direct("solhint"),
    CommandType::Npm("solhint"),
    CommandType::Pnpm("solhint"),
    CommandType::Bun("solhint"),
    CommandType::Deno("solhint"),
    CommandType::Yarn("solhint"),
];

pub const IS_STDIN: bool = false;
