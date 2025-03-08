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
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("ansible-lint"),
    CommandType::Uv("ansible-dev-tools"),
    CommandType::Pipx("ansible-dev-tools"),
];

pub const IS_STDIN: bool = false;
