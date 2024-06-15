use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_markuplint_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--fix").arg(snippet_path);
}

#[inline]
fn invoke_markuplint(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_markuplint_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn format_using_markuplint(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_markuplint(std::process::Command::new("markuplint"), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_markuplint(setup_npm_script("markuplint"), snippet_path)
}
