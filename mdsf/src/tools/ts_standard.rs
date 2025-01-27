///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_ts_standard_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("ts-standard"),
        CommandType::Direct("ts-standard"),
        CommandType::Npm("ts-standard"),
    ];

    crate::execution::run_tools(&commands, file_path, timeout, set_ts_standard_args)
}

#[cfg(test)]
mod test_ts_standard {}
