use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_prisma_args(cmd: &mut tokio::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("format")
        .arg(format!("--schema={}", snippet_path.display()));
}

#[inline]
async fn invoke_prisma(
    mut cmd: tokio::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_prisma_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[inline]
pub async fn run_format(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_prisma(tokio::process::Command::new("prisma"), snippet_path).await
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_prisma(setup_npm_script("prisma"), snippet_path).await
}
