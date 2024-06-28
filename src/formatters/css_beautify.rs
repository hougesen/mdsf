use super::execute_command;
use crate::{error::MdsfError, runners::run_executable_from_path};

#[inline]
fn set_css_beautify_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-r")
        .arg("--type")
        .arg("css")
        .arg("-f")
        .arg(snippet_path);

    cmd
}

#[inline]
fn invoke_css_beautify(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_css_beautify_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_css_beautify(
        run_executable_from_path("node_modules/.bin/css-beautify"),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_css_beautify(std::process::Command::new("css-beautify"), snippet_path)
}
