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

        crate::tools::Tooling::Pyflakes.test_format_snippet(input, output, &file_ext);
    }
}
