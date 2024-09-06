use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_nph_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(snippet_path);

    cmd
}

#[inline]
fn invoke_nph(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_nph_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    invoke_nph(CommandType::Direct("nph").build(), snippet_path)
}
