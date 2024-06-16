use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("dprint");

    cmd.arg("fmt").arg(file_path);

    execute_command(&mut cmd, file_path)
}
