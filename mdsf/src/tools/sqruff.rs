///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fix");
    cmd.arg("--force");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("sqruff"),
    CommandType::Uv("sqruff", "sqruff"),
    CommandType::Pipx("sqruff"),
];

pub const IS_STDIN: bool = false;
