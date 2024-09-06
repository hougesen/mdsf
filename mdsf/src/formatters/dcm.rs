use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run_format(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("dcm").build();

    cmd.arg("format").arg(file_path);

    execute_command(cmd, file_path)
}

#[inline]
pub fn run_fix(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("dcm").build();

    cmd.arg("fix").arg(file_path);

    execute_command(cmd, file_path)
}
