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

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("isort"),
    CommandType::Uv("isort", "isort"),
    CommandType::Pipx("isort"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_isort {
    #[test_with::executable(isort || pipx || uv)]
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

        crate::tools::Tooling::Isort.test_format_snippet(input, output, &file_ext);
    }
}
