///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_terraform_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("fmt");
    cmd.arg("-write=true");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("terraform")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_terraform_fmt_args(cmd.build(), file_path);
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
mod test_terraform_fmt {
    #[test_with::executable(terraform)]
    fn test_terraform_fmt_tf_c8abf8bba550fe29() {
        let input = r#"resource "aws_instance" "example" {
                ami   = "abc123"

           network_interface  {
             }
}
"#;
        let output = Some(
            r#"resource "aws_instance" "example" {
  ami = "abc123"

  network_interface {
  }
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("tf");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::terraform_fmt::run(snippet.path())
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
