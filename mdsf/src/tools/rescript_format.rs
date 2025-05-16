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
    CommandType::NodeModules("rescript"),
    CommandType::Direct("rescript"),
    CommandType::Npm("rescript"),
    CommandType::Pnpm("rescript"),
    CommandType::Bun("rescript"),
    CommandType::Deno("rescript"),
    CommandType::Yarn("rescript"),
];

pub const IS_STDIN: bool = false;
