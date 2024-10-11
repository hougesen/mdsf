use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run_format(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("mojo").build();

    cmd.arg("format").arg("-q").arg(snippet_path);

    execute_command(cmd, snippet_path)
}
