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
    CommandType::Direct("pyink"),
    CommandType::Uv("pyink", "pyink"),
    CommandType::Pipx("pyink"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_pyink {
    #[test_with::executable(pyink || pipx || uv)]
    fn test_pyink_python_229ec2b01c2bfe3c() {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::Pyink.test_format_snippet(input, output, &file_ext);
    }
}
