///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fmt");
    cmd.arg("--stdin");
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("mise"),
    CommandType::Direct("mise"),
    CommandType::Npm("@jdxcode/mise"),
    CommandType::Pnpm("@jdxcode/mise"),
    CommandType::Bun("@jdxcode/mise"),
    CommandType::Deno("@jdxcode/mise"),
    CommandType::Yarn("@jdxcode/mise"),
];

pub const IS_STDIN: bool = true;
