use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_eslint_args(cmd: &mut tokio::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--fix").arg(snippet_path);
}

#[inline]
async fn invoke_eslint(
    mut cmd: tokio::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_eslint_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_eslint(tokio::process::Command::new("eslint"), snippet_path).await
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_eslint(setup_npm_script("eslint"), snippet_path).await
}
