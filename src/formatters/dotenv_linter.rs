use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("dotenv-linter");

    cmd.arg("fix").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}