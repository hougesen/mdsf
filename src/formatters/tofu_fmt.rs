use super::execute_command;
use crate::terminal::print_formatter_info;

#[inline]
pub fn format_using_tofu_fmt(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("tofu_fmt");

    let mut cmd = std::process::Command::new("tofu");

    cmd.arg("fmt").arg("-write=true").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_tofu_fmt {
    use super::format_using_tofu_fmt;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(tofu)]
    #[test]
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

        let snippet =
            setup_snippet(input, Language::Hcl.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_tofu_fmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
