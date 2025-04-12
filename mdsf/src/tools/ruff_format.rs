///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
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
mod test_ruff_format {
    #[test_with::executable(ruff || pipx || uv)]
    fn test_ruff_format_python_229ec2b01c2bfe3c() {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::RuffFormat.test_format_snippet(input, output, &file_ext);
    }
}
