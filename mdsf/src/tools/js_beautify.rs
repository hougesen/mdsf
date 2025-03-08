///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-r");
    cmd.arg("--type");
    cmd.arg("js");
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 6] = [
    CommandType::NodeModules("js-beautify"),
    CommandType::Direct("js-beautify"),
    CommandType::Npm("js-beautify"),
    CommandType::Pnpm("js-beautify"),
    CommandType::Bun("js-beautify"),
    CommandType::Deno("js-beautify"),
];

pub const IS_STDIN: bool = false;
