use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_scalafmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--quiet");
    cmd.arg("--mode");
    cmd.arg("any");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("scalafmt")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_scalafmt_args(cmd.build(), file_path);
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
mod test_scalafmt {
    #[test_with::executable(scalafmt)]
    fn test_scalafmt_scala_f3726f9b9a0f9066() {
        let input = r#"object Addition {
             def main() = {
                 println(1 + 3)
             }
    }"#;
        let output = r#"object Addition {
  def main() = {
    println(1 + 3)
  }
}
"#;
        let file_ext = crate::fttype::get_file_extension("scala");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::scalafmt::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
