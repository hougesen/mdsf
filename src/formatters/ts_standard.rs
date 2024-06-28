use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_ts_standard_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_ts_standard(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_ts_standard_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_ts_standard(
        CommandType::NodeModules("ts-standard").build(),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) =
        invoke_ts_standard(CommandType::Direct("ts-standard").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_ts_standard(CommandType::Npm("ts-standard").build(), snippet_path)
}
