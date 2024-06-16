use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_xo_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--fix").arg(snippet_path);
}

#[inline]
fn invoke_xo(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_xo_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_xo(std::process::Command::new("xo"), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_xo(setup_npm_script("xo"), snippet_path)
}
