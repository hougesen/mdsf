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
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("ufmt"),
    CommandType::Uv("ufmt", "ufmt"),
    CommandType::Pipx("ufmt"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_ufmt {
    #[test_with::executable(ufmt || pipx || uv)]
    fn test_ufmt_python_229ec2b01c2bfe3c() {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::Ufmt.test_format_snippet(input, output, &file_ext);
    }
}
