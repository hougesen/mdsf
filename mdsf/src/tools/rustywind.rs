///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("rustywind"),
    CommandType::Direct("rustywind"),
    CommandType::Npm("rustywind"),
    CommandType::Pnpm("rustywind"),
    CommandType::Bun("rustywind"),
    CommandType::Deno("rustywind"),
    CommandType::Yarn("rustywind"),
];

pub const IS_STDIN: bool = false;
