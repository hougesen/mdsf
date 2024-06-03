use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_docstrfmt(
    file_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("docstrfmt");

    cmd.arg(file_path);

    execute_command(&mut cmd, file_path)
}
