use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_forge_fmt(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("forge");

    cmd.arg("fmt").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}