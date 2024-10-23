use std::process::Command;

use crate::{error::MdsfError, formatters::execute_command, runners::CommandType};

#[inline]
fn set_prettier_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format").arg("--write").arg(file_path);

    cmd
}

#[inline]
pub fn run_prettier(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::NodeModules("prettier"), CommandType::Direct("prettier"), CommandType::Npm("prettier")];


    for (index, c) in commands.iter().enumerate() {
        let mut cmd = c.build();

        let execution_result = execute_command(set_prettier_args(cmd, file_path), file_path);

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
