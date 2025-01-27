///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_tofu_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("fmt");
    cmd.arg("-write=true");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("tofu")];

    crate::execution::run_tools(&commands, file_path, timeout, set_tofu_fmt_args)
}

#[cfg(test)]
mod test_tofu_fmt {
    #[test_with::executable(tofu)]
    fn test_tofu_fmt_terraform_d0328fd989abae6d() {
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
        let file_ext = crate::fttype::get_file_extension("terraform");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::tofu_fmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
