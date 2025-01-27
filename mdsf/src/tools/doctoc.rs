///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
fn set_doctoc_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("doctoc"),
    CommandType::Direct("doctoc"),
    CommandType::Npm("doctoc"),
];

#[inline]
pub fn run(
    file_path: &std::path::Path,
    timeout: u64,
) -> Result<(bool, Option<String>), crate::error::MdsfError> {
    crate::execution::run_tools(&COMMANDS, file_path, timeout, set_doctoc_args)
}

#[cfg(test)]
mod test_doctoc {}
