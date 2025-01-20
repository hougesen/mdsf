use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_csslint_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("csslint"),
        CommandType::Direct("csslint"),
        CommandType::Npm("csslint"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_csslint_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path).map(|value| (value.0, None));

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
mod test_csslint {}
