///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg("--write-stdout");
    cmd
}

pub const COMMANDS: [CommandType; 2] = [
    CommandType::Dotnet("csharpier"),
    CommandType::Direct("csharpier"),
];

pub const IS_STDIN: bool = true;
