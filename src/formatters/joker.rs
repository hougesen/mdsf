use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_joker(
    file_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("joker");

    cmd.arg("--format").arg("--write").arg(file_path);

    execute_command(&mut cmd, file_path)
}
