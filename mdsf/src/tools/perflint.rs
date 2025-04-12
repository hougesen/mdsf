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
    CommandType::Direct("perflint"),
    CommandType::Uv("perflint", "perflint"),
    CommandType::Pipx("perflint"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_perflint {
    #[test_with::executable(perflint || pipx || uv)]
    fn test_perflint_python_2a683a1c25614024() {
        let input = r#"def add(a, b): return a + b"#;

        let output = r#"def add(a, b): return a + b"#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::Perflint.test_format_snippet(input, output, &file_ext);
    }
}
