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
    CommandType::NodeModules("blade-formatter"),
    CommandType::Direct("blade-formatter"),
    CommandType::Npm("blade-formatter"),
    CommandType::Pnpm("blade-formatter"),
    CommandType::Bun("blade-formatter"),
    CommandType::Deno("blade-formatter"),
    CommandType::Yarn("blade-formatter"),
];

pub const IS_STDIN: bool = false;
