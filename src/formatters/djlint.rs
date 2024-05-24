use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_djlint(
    file_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("djlint");

    cmd.arg(file_path).arg("--reformat");

    execute_command(&mut cmd, file_path)
}
