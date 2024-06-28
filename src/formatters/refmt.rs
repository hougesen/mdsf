use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_refmt_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--in-place").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_refmt(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_refmt_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    invoke_refmt(CommandType::Direct("refmt").build(), snippet_path)
}
