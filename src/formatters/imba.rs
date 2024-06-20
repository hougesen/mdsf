use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run_fmt(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("imba");

    cmd.arg("fmt").arg("-f").arg(file_path);

    execute_command(&mut cmd, file_path)
}
