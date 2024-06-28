use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_js_beautify_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-r")
        .arg("--type")
        .arg("js")
        .arg("-f")
        .arg(snippet_path);

    cmd
}

#[inline]
fn invoke_js_beautify(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_js_beautify_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_js_beautify(
        CommandType::NodeModules("js-beautify").build(),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_js_beautify(CommandType::Direct("js-beautify").build(), snippet_path)
}
