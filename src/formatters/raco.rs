use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run_fmt(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("raco").build();

    cmd.arg("fmt").arg("-i").arg(file_path);

    execute_command(cmd, file_path)
}
