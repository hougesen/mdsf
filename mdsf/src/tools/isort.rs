///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("isort")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_isort {
    #[test_with::executable(isort)]
    fn test_isort_python_e2ac93e0195d9bc1() {
        let input = r#"from q import d
import b
import a
import c


def add(a: int, b: int) -> int:
  return a + b
"#;

        let output = r#"import a
import b
import c
from q import d


def add(a: int, b: int) -> int:
  return a + b
"#;

        let file_ext = crate::fttype::get_file_extension("python");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Isort
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}
