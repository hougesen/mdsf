use super::execute_command;
use crate::{
    error::MdsfError,
    runners::{run_executable_from_path, setup_npm_script},
};

#[inline]
fn set_oxlint_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_oxlint(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_oxlint_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_oxlint(
        run_executable_from_path("node_modules/.bin/oxlint"),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) = invoke_oxlint(std::process::Command::new("oxlint"), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_oxlint(setup_npm_script("oxlint"), snippet_path)
}
