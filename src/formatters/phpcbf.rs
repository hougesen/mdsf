use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_phpcbf_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(snippet_path);

    cmd
}

#[inline]
fn invoke_phpcbf(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_phpcbf_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_phpcbf(CommandType::PhpVendor("phpcbf").build(), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_phpcbf(CommandType::Direct("phpcbf").build(), snippet_path)
}
