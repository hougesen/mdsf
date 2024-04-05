use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_stylelint_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--fix").arg(snippet_path);
}

#[inline]
fn invoke_stylelint(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_stylelint_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn format_using_stylelint(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let global_result = invoke_stylelint(std::process::Command::new("stylelint"), snippet_path)?;

    if !global_result.0 {
        return Ok(global_result);
    }

    invoke_stylelint(setup_npm_script("stylelint"), snippet_path)
}
