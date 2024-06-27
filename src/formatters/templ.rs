use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("templ");

    cmd.arg("fmt").arg(file_path);

    execute_command(cmd, file_path)
}
