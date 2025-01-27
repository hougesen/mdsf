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
    CommandType::NodeModules("quick-lint-js"),
    CommandType::Direct("quick-lint-js"),
    CommandType::Npm("quick-lint-js"),
];

#[cfg(test)]
mod test_quick_lint_js {}
