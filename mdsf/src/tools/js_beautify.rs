///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_arguments(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-r");
    cmd.arg("--type");
    cmd.arg("js");
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("js-beautify"),
    CommandType::Direct("js-beautify"),
    CommandType::Npm("js-beautify"),
];

#[cfg(test)]
mod test_js_beautify {}
