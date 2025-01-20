///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

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
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::PhpVendor("twig-cs-fixer"),
        CommandType::Direct("twig-cs-fixer"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_twig_cs_fixer_lint_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path);

        if index == commands.len() - 1 {
            return execution_result;
        }

        if let Ok(r) = execution_result {
            if !r.0 {
                return Ok(r);
            }
        }
    }

    Ok((true, None))
}

#[cfg(test)]
mod test_twig_cs_fixer_lint {}
