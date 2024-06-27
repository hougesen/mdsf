use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_stylelint_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_stylelint(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_stylelint_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_stylelint(std::process::Command::new("stylelint"), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_stylelint(setup_npm_script("stylelint"), snippet_path)
}
