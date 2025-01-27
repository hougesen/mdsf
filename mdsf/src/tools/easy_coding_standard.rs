///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_easy_coding_standard_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("check");
    cmd.arg(file_path);
    cmd.arg("--fix");
    cmd.arg("--no-interaction");
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::PhpVendor("ecs"), CommandType::Direct("ecs")];

    crate::execution::run_tools(&commands, file_path, timeout, set_easy_coding_standard_args)
}

#[cfg(test)]
mod test_easy_coding_standard {}
