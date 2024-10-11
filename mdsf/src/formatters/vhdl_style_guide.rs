use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("vsg").build();

    cmd.arg("-f").arg(snippet_path).arg("--fix");

    execute_command(cmd, snippet_path)
}
