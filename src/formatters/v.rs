use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async  fn run_fmt(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("v");

    cmd.arg("fmt").arg("-w").arg(snippet_path);

    execute_command(&mut cmd, snippet_path).await
}
