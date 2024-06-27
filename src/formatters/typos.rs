use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("typos");

    cmd.arg("-w").arg("--hidden").arg(snippet_path);

    execute_command(cmd, snippet_path)
}
