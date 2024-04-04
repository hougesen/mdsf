use super::execute_command;
use crate::terminal::print_formatter_info;

#[inline]
pub fn format_using_isort(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("isort");

    let mut cmd = std::process::Command::new("isort");

    cmd.arg("--quiet").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_isort {
    use super::format_using_isort;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(isort)]
    #[test]
    fn it_should_format_python() {
        let input = "from q import d
import b
import a
import c


def add(a: int, b: int) -> int:
  return a + b
";

        let expected_output = "import a
import b
import c
from q import d


def add(a: int, b: int) -> int:
  return a + b
";

        let snippet = setup_snippet(input, Language::Python.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_isort(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
