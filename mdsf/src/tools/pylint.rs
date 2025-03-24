///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--module-naming-style=any");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("pylint"),
    CommandType::Uv("pylint", "pylint"),
    CommandType::Pipx("pylint"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_pylint {
    #[test_with::executable(pylint || pipx || uv)]
    fn test_pylint_python_826209940b0fafbc() {
        let input = r#""""
mdsf test module for pylint
"""


def add(a: int, b: int) -> int:
    """
    Add the numbers
    """
    return a + b
"#;

        let output = r#""""
mdsf test module for pylint
"""


def add(a: int, b: int) -> int:
    """
    Add the numbers
    """
    return a + b
"#;

        let file_ext = crate::fttype::get_file_extension("python");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Pylint
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
