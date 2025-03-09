///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fmt");
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("imba"),
    CommandType::Direct("imba"),
    CommandType::Npm("imba"),
    CommandType::Pnpm("imba"),
    CommandType::Bun("imba"),
    CommandType::Deno("imba"),
    CommandType::Yarn("imba"),
];

pub const IS_STDIN: bool = false;
