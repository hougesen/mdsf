///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("pyflakes"),
    CommandType::Uv("pyflakes", "pyflakes"),
    CommandType::Pipx("pyflakes"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_pyflakes {
    #[test_with::executable(pyflakes || pipx || uv)]
    fn test_pyflakes_python_8c5d8d3b8d3870d1() {
        let input = r#"def add(a: int, b: int) -> int:
    return a + b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b"#;

        let file_ext = crate::fttype::get_file_extension("python");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Pyflakes
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
