use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_ts_standard_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--fix").arg(snippet_path);
}

#[inline]
fn invoke_ts_standard(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_ts_standard_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn format_using_ts_standard(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_ts_standard(std::process::Command::new("ts-standard"), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_ts_standard(setup_npm_script("ts-standard"), snippet_path)
}
