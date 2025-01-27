///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_arguments(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fmt");
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("imba"),
    CommandType::Direct("imba"),
    CommandType::Npm("imba"),
];

#[cfg(test)]
mod test_imba_fmt {}
