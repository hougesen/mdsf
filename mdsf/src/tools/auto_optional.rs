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
    CommandType::Direct("auto-optional"),
    CommandType::Uv("auto-optional", "auto-optional"),
    CommandType::Pipx("auto-optional"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_auto_optional {
    #[test_with::executable(auto-optional || pipx || uv)]
    fn test_auto_optional_python_c43199b18f48026d() {
        let input = r#"def foo(bar: str = None):
    pass
"#;

        let output = r#"from typing import Optional
def foo(bar: Optional[str] = None):
    pass
"#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::AutoOptional.test_format_snippet(input, output, &file_ext);
    }
}
