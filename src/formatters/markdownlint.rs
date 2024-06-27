use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("markdownlint");

    cmd.arg("--fix").arg(file_path);

    execute_command(&mut cmd, file_path).await }
