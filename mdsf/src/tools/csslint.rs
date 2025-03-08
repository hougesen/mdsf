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

pub const COMMANDS: [CommandType; 6] = [
    CommandType::NodeModules("csslint"),
    CommandType::Direct("csslint"),
    CommandType::Npm("csslint"),
    CommandType::Pnpm("csslint"),
    CommandType::Bun("csslint"),
    CommandType::Deno("csslint"),
];

pub const IS_STDIN: bool = false;
