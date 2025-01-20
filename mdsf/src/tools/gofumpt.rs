use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_gofumpt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-w");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("gofumpt")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_gofumpt_args(cmd.build(), file_path);
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
mod test_gofumpt {
    #[test_with::executable(gofumpt)]
    fn test_gofumpt_go_3b56f602fe22977b() {
        let input = r#"package main

   func add(a int , b int  ) int {
                return a + b
       }

    "#;
        let output = r#"package main

func add(a int, b int) int {
	return a + b
}
"#;
        let file_ext = crate::fttype::get_file_extension("go");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::gofumpt::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
