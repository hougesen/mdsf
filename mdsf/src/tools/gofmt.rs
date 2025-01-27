///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_gofmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-w");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("gofmt")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_gofmt_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path, timeout);

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
mod test_gofmt {
    #[test_with::executable(gofmt)]
    fn test_gofmt_go_e8804a3fa960a187() {
        let input = r#"package main

   func add(a int , b int  ) int {
                return a + b
       }

    "#;
        let output = Some(
            r#"package main

func add(a int, b int) int {
	return a + b
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("go");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::gofmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
