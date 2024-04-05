use super::execute_command;

#[inline]
pub fn format_using_usort(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    let mut cmd = std::process::Command::new("usort");

    cmd.arg("format").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_usort {
    use super::format_using_usort;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(usort)]
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

        let output = format_using_usort(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
