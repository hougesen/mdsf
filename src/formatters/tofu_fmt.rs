use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("tofu");

    cmd.arg("fmt").arg("-write=true").arg(snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[cfg(test)]
mod test_tofu_fmt {
    use crate::formatters::setup_snippet;

    #[tokio::test]
    #[test_with::executable(tofu)]
    async fn it_should_format_hcl() {
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

        let snippet = setup_snippet(input, ".tf")
            .await
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
