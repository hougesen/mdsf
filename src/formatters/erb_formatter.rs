use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_erb_formatter(
    file_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("erb-format");

    cmd.arg(file_path).arg("--write");

    execute_command(&mut cmd, file_path)
}
