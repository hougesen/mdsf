use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_pint_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(snippet_path);

    cmd
}

#[inline]
fn invoke_pint(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_pint_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_pint(CommandType::PhpVendor("pint").build(), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_pint(CommandType::Direct("pint").build(), snippet_path)
}
