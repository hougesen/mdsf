use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_fixjson(
    file_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("fixjson");

    cmd.arg("-w").arg(file_path);

    execute_command(&mut cmd, file_path)
}
