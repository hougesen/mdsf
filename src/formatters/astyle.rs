use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("astyle").build();

    cmd.arg("--quiet").arg(snippet_path);

    execute_command(cmd, snippet_path)
}
