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
    cmd.arg("--check-hidden");
    cmd.arg("--write-changes");
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("codespell")];

pub const IS_STDIN: bool = false;
