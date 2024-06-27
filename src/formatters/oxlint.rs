use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_oxlint_args(cmd: &mut tokio::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--fix").arg(snippet_path);
}

#[inline]
async fn invoke_oxlint(
    mut cmd: tokio::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_oxlint_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_oxlint(tokio::process::Command::new("oxlint"), snippet_path).await
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_oxlint(setup_npm_script("oxlint"), snippet_path).await
}
