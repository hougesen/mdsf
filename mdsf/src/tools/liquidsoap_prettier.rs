///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_liquidsoap_prettier_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("liquidsoap-prettier"),
        CommandType::Direct("liquidsoap-prettier"),
        CommandType::Npm("liquidsoap-prettier"),
    ];

    crate::execution::run_tools(&commands, file_path, timeout, set_liquidsoap_prettier_args)
}

#[cfg(test)]
mod test_liquidsoap_prettier {}
