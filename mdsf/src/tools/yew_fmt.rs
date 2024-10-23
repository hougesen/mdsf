use std::process::Command;

use crate::{error::MdsfError, formatters::execute_command, runners::CommandType};

#[inline]
fn set_yew_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--edition");
    cmd.arg("2021");
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("yew-fmt")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_yew_fmt_args(cmd.build(), file_path);
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
