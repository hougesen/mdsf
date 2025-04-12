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
    CommandType::Direct("cpplint"),
    CommandType::Uv("cpplint", "cpplint"),
    CommandType::Pipx("cpplint"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_cpplint {
    #[test_with::executable(cpplint || pipx || uv)]
    fn test_cpplint_cpp_5edac26b16656f() {
        let input = r#"// Copyright 2025 Mads Hougesen
int add(int a, int b) { return a + b; }
"#;

        let output = r#"// Copyright 2025 Mads Hougesen
int add(int a, int b) { return a + b; }
"#;

        let file_ext = crate::fttype::get_file_extension("cpp");

        crate::tools::Tooling::Cpplint.test_format_snippet(input, output, &file_ext);
    }
}
