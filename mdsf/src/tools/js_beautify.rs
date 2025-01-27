///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_js_beautify_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-r");
    cmd.arg("--type");
    cmd.arg("js");
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("js-beautify"),
        CommandType::Direct("js-beautify"),
        CommandType::Npm("js-beautify"),
    ];

    crate::execution::run_tools(&commands, file_path, timeout, set_js_beautify_args)
}

#[cfg(test)]
mod test_js_beautify {}
