use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("isort").build();

    cmd.arg("--quiet").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_isort {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(isort)]
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

        let snippet = setup_snippet(input, &get_file_extension("python"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
