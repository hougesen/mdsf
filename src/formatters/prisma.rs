use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_prisma_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format")
        .arg(format!("--schema={}", snippet_path.display()));

    cmd
}

#[inline]
fn invoke_prisma(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_prisma_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run_format(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_prisma(std::process::Command::new("prisma"), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_prisma(setup_npm_script("prisma"), snippet_path)
}
