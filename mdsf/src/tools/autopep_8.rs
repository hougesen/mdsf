///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--in-place");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("autopep8"),
    CommandType::Uv("autopep8", "autopep8"),
    CommandType::Pipx("autopep8"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_autopep_8 {
    #[test_with::executable(autopep8 || pipx || uv)]
    fn test_autopep_8_python_a868b5ad9905fc3f() {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int,  b: int) -> int: return a+b
"#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::Autopep8.test_format_snippet(input, output, &file_ext);
    }
}
