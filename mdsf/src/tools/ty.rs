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
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("ty"),
    CommandType::Uv("ty", "ty"),
    CommandType::Pipx("ty"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_ty {
    #[test_with::executable(ty || pipx || uv)]
    fn test_ty_python_13af245604aaa0cd() {
        let input = r#"def add(a: int, b: int) -> int:
    return a + b


add(1, 2)
"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b


add(1, 2)
"#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::Ty.test_format_snippet(input, output, &file_ext);
    }
}
