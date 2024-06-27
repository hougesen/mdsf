use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_ts_standard_args(cmd: &mut tokio::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--fix").arg(snippet_path);
}

#[inline]
async fn invoke_ts_standard(
    mut cmd: tokio::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_ts_standard_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_ts_standard(tokio::process::Command::new("ts-standard"), snippet_path).await
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_ts_standard(setup_npm_script("ts-standard"), snippet_path).await
}
