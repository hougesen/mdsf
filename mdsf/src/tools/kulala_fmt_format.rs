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

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("kulala-fmt"),
    CommandType::Direct("kulala-fmt"),
    CommandType::Npm("@mistweaverco/kulala-fmt"),
];

#[cfg(test)]
mod test_kulala_fmt_format {}
