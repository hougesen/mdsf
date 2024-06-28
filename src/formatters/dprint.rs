use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_dprint_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fmt").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_dprint(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_dprint_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_dprint(CommandType::NodeModules("dprint").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_dprint(CommandType::Direct("dprint").build(), snippet_path)
}
