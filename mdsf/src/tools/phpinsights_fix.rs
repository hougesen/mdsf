///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
fn set_phpinsights_fix_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fix");
    cmd.arg(file_path);
    cmd.arg("--no-interaction");
    cmd.arg("--quiet");
    cmd
}

const COMMANDS: [CommandType; 2] = [
    CommandType::PhpVendor("phpinsights"),
    CommandType::Direct("phpinsights"),
];

#[inline]
pub fn run(
    file_path: &std::path::Path,
    timeout: u64,
) -> Result<(bool, Option<String>), crate::error::MdsfError> {
    crate::execution::run_tools(&COMMANDS, file_path, timeout, set_phpinsights_fix_args)
}

#[cfg(test)]
mod test_phpinsights_fix {}
