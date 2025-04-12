///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("check");
    cmd.arg("--fix");
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("ruff"),
    CommandType::Uv("ruff", "ruff"),
    CommandType::Pipx("ruff"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_ruff_check {
    #[test_with::executable(ruff || pipx || uv)]
    fn test_ruff_check_python_e2f9361cc55100c5() {
        let input = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::RuffCheck.test_format_snippet(input, output, &file_ext);
    }
}
