use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_gci(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("gci");

    cmd.arg("write")
        .arg("--skip-generated")
        .arg("--skip-vendor")
        .arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}
