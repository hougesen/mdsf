use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("bsfmt");

    cmd.arg(file_path).arg("--write");

    execute_command(&mut cmd, file_path).await
}
