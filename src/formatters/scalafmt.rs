use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("scalafmt");

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

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_scalafmt {
    use super::run;
    use crate::{formatters::setup_snippet, generated::language_to_ext};

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

        let snippet =
            setup_snippet(input, language_to_ext("scala")).expect("it to create a snippet file");

        let output = run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
