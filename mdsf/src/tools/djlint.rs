///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_arguments(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd.arg("--reformat");
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("djlint")];

#[cfg(test)]
mod test_djlint {}
