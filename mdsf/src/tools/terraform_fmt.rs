///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_terraform_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("fmt");
    cmd.arg("-write=true");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("terraform")];

    crate::execution::run_tools(&commands, file_path, timeout, set_terraform_fmt_args)
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
        let result = crate::tools::terraform_fmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
