use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("gersemi").build();

    cmd.arg("-i").arg("-q").arg(file_path);

    execute_command(cmd, file_path)
}
