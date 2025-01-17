///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_tofu_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("fmt");
    cmd.arg("-write=true");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("tofu")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_tofu_fmt_args(cmd.build(), file_path);
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
mod test_tofu_fmt {
    #[test_with::executable(tofu)]
    fn test_tofu_fmt_terraform_ad45c247a9c563a1() {
        let input = r#"resource "aws_instance" "example" {
                ami   = "abc123"

           network_interface  {
             }
}
"#;
        let output = r#"resource "aws_instance" "example" {
  ami = "abc123"

  network_interface {
  }
}
"#;
        let file_ext = crate::fttype::get_file_extension("terraform");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::tofu_fmt::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
