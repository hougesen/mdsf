use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("tofu");

    cmd.arg("fmt").arg("-write=true").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_tofu_fmt {
    use super::run;
    use crate::formatters::setup_snippet;

    #[test_with::executable(tofu)]
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

        let output = run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
