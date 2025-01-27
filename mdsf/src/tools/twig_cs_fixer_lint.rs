///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_twig_cs_fixer_lint_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("lint");
    cmd.arg(file_path);
    cmd.arg("--fix");
    cmd.arg("--no-interaction");
    cmd.arg("--quiet");
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::PhpVendor("twig-cs-fixer"),
        CommandType::Direct("twig-cs-fixer"),
    ];

    crate::execution::run_tools(&commands, file_path, timeout, set_twig_cs_fixer_lint_args)
}

#[cfg(test)]
mod test_twig_cs_fixer_lint {}
