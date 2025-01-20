use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_beautysh_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("beautysh")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_beautysh_args(cmd.build(), file_path);
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
mod test_beautysh {
    #[test_with::executable(beautysh)]
    fn test_beautysh_bash_a6831a7ad31bd0a6() {
        let input = r#"#!/bin/bash

       add() {
    echo "$1" + "$2"
             }
"#;
        let output = r#"#!/bin/bash

add() {
    echo "$1" + "$2"
}
"#;
        let file_ext = crate::fttype::get_file_extension("bash");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::beautysh::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }

    #[test_with::executable(beautysh)]
    fn test_beautysh_shell_f8c934ee37e2888() {
        let input = r#"#!/bin/shell

       add() {
    echo "$1" + "$2"
             }
"#;
        let output = r#"#!/bin/shell

add() {
    echo "$1" + "$2"
}
"#;
        let file_ext = crate::fttype::get_file_extension("shell");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::beautysh::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
