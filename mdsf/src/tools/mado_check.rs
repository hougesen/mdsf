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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("mado")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_mado_check {
    #[test_with::executable(mado)]
    fn test_mado_check_markdown_664925a21a5aec00() {
        let input = r#"# Hello world

- Hello
- world

"#;

        let output = r#"# Hello world

- Hello
- world

"#;

        let file_ext = crate::fttype::get_file_extension("markdown");

        crate::tools::Tooling::MadoCheck.test_format_snippet(input, output, &file_ext);
    }
}
