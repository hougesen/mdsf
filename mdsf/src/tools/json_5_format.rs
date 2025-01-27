///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_json_5_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-r");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("json5format")];

    crate::execution::run_tools(&commands, file_path, timeout, set_json_5_format_args)
}

#[cfg(test)]
mod test_json_5_format {}
