use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_oxlint_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--fix").arg(snippet_path);
}

#[inline]
fn invoke_oxlint(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_oxlint_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn format_using_oxlint(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_oxlint(std::process::Command::new("oxlint"), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_oxlint(setup_npm_script("oxlint"), snippet_path)
}
