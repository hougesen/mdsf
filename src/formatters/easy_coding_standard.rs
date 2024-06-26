use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_ecs_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("check")
        .arg(snippet_path)
        .arg("--fix")
        .arg("--no-interaction");

    cmd
}

#[inline]
fn invoke_ecs(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_ecs_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_ecs(CommandType::PhpVendor("ecs").build(), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_ecs(CommandType::Direct("ecs").build(), snippet_path)
}
