use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_xo_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_xo(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_xo_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_xo(CommandType::NodeModules("xo").build(), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) = invoke_xo(CommandType::Direct("xo").build(), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_xo(CommandType::Npm("xo").build(), snippet_path)
}
