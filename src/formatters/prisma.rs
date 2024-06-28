use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

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
    if let Ok(path_result) = invoke_prisma(CommandType::NodeModules("prisma").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) = invoke_prisma(CommandType::Direct("prisma").build(), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_prisma(CommandType::Npm("prisma").build(), snippet_path)
}
