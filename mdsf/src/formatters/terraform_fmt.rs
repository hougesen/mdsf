use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("terraform").build();

    cmd.arg("fmt").arg("-write=true").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_terraform_fmt {
    use crate::formatters::setup_snippet;

    #[test_with::executable(terraform)]
    fn it_should_format_hcl() {
        let input = "resource \"aws_instance\" \"example\" {
                    ami   = \"abc123\"

           network_interface  {
             }
}
";

        let expected_output = "resource \"aws_instance\" \"example\" {
  ami = \"abc123\"

  network_interface {
  }
}
";

        let snippet = setup_snippet(input, ".tf").expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
