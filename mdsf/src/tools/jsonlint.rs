///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_arguments(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("jsonlint"),
    CommandType::Direct("jsonlint"),
    CommandType::Npm("jsonlint"),
];

#[cfg(test)]
mod test_jsonlint {}
