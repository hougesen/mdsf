///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd.arg("--write");
    cmd
}

pub const COMMANDS: [CommandType; 6] = [
    CommandType::NodeModules("bsfmt"),
    CommandType::Direct("bsfmt"),
    CommandType::Npm("brighterscript-formatter"),
    CommandType::Pnpm("brighterscript-formatter"),
    CommandType::Bun("brighterscript-formatter"),
    CommandType::Deno("brighterscript-formatter"),
];

pub const IS_STDIN: bool = false;
