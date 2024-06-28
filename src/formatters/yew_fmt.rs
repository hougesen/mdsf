use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("yew-fmt").build();

    // Needed for async
    cmd.arg("--edition").arg("2021");

    cmd.arg("--quiet").arg(snippet_path);

    execute_command(cmd, snippet_path)
}
