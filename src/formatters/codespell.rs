use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_codespell(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("codespell");

    cmd.arg(snippet_path).arg("--check-hidden");

    execute_command(&mut cmd, snippet_path)
}
