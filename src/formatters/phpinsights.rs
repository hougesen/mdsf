use super::execute_command;
use crate::{error::MdsfError, runners::run_executable_from_path};

#[inline]
fn set_phpinsights_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fix")
        .arg(snippet_path)
        .arg("--no-interaction")
        .arg("--quiet");

    cmd
}

#[inline]
fn invoke_phpinsights(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_phpinsights_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_phpinsights(
        run_executable_from_path("vendor/bin/phpinsights"),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_phpinsights(std::process::Command::new("phpinsights"), snippet_path)
}
