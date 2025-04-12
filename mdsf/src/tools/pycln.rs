///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--no-gitignore");
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("pycln"),
    CommandType::Uv("pycln", "pycln"),
    CommandType::Pipx("pycln"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_pycln {
    #[test_with::executable(pycln || pipx || uv)]
    fn test_pycln_python_21e4539a9b183542() {
        let input = r#"import math"#;

        let output = r#""#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::Pycln.test_format_snippet(input, output, &file_ext);
    }
}
