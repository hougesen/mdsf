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
    cmd.arg("--reformat");
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("djlint"),
    CommandType::Uv("djlint"),
    CommandType::Pipx("djlint"),
];

pub const IS_STDIN: bool = false;
