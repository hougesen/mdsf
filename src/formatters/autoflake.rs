use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_autoflake(
    file_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("autoflake");

    cmd.arg("--in-place").arg(file_path);

    execute_command(&mut cmd, file_path)
}