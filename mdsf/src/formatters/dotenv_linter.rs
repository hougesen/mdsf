use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("dotenv-linter").build();

    cmd.arg("fix").arg(snippet_path);

    execute_command(cmd, snippet_path)
}
