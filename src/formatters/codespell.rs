use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("codespell");

    cmd.arg(snippet_path)
        .arg("--check-hidden")
        .arg("--write-changes");

    execute_command(&mut cmd, snippet_path).await
}
