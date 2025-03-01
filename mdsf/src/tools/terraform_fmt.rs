///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fmt");
    cmd.arg("-write=true");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("terraform")];

#[cfg(test)]
mod test_terraform_fmt {
    const TIMEOUT: u64 = 0;
    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(terraform)]
    fn test_terraform_fmt_tf_2c1d9f26008080c1() {
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

        let file_ext = crate::fttype::get_file_extension("tf");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            false,
            DEBUG_ENABLED,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
