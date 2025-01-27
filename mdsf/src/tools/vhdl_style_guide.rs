///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_vhdl_style_guide_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd.arg("--fix");
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("vsg")];

    crate::execution::run_tools(&commands, file_path, timeout, set_vhdl_style_guide_args)
}

#[cfg(test)]
mod test_vhdl_style_guide {}
