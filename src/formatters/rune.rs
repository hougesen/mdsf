use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run_fmt(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("rune").build();

    cmd.arg("fmt").arg(file_path);

    execute_command(cmd, file_path)
}
