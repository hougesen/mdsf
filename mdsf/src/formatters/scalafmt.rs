use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("scalafmt").build();

    #[cfg(test)]
    {
        cmd.arg("--config-str")
            .arg("\"version=3.8.0\"")
            .arg("--debug");
    };

    cmd.arg("--quiet")
        .arg("--mode")
        .arg("any")
        .arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_scalafmt {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(scalafmt)]
    fn it_should_format_scala() {
        let input = "object Addition {
             def main() = {
                 println(1 + 3)
             }
    }";
        let expected_output = "object Addition {
  def main() = {
    println(1 + 3)
  }
}
";

        let snippet = setup_snippet(input, &get_file_extension("scala"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
