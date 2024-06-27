use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("djlint");

    cmd.arg(file_path).arg("--reformat");

    execute_command(cmd, file_path)
}
