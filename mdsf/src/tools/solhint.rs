///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
fn set_solhint_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--quiet");
    cmd.arg("--fix");
    cmd.arg("--noPrompt");
    cmd.arg(file_path);
    cmd
}

const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("solhint"),
    CommandType::Direct("solhint"),
    CommandType::Npm("solhint"),
];

#[inline]
pub fn run(
    file_path: &std::path::Path,
    timeout: u64,
) -> Result<(bool, Option<String>), crate::error::MdsfError> {
    crate::execution::run_tools(&COMMANDS, file_path, timeout, set_solhint_args)
}

#[cfg(test)]
mod test_solhint {}
