///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-r");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("unimport"),
    CommandType::Uv("unimport", "unimport"),
    CommandType::Pipx("unimport"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_unimport {
    #[test_with::executable(unimport || pipx || uv)]
    fn test_unimport_python_3940fba56a9a47fc() {
        let input = r#"from typing import Optional
"#;

        let output = r#"
"#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::Unimport.test_format_snippet(input, output, &file_ext);
    }
}
