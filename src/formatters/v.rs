use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run_fmt(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("v");

    cmd.arg("fmt").arg("-w").arg(snippet_path);

    execute_command(cmd, snippet_path)
}
