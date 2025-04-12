///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-w");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("misspell")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_misspell {
    #[test_with::executable(misspell)]
    fn test_misspell_markdown_bf3aae6887d87bbc() {
        let input = r#"langauge"#;

        let output = r#"language"#;

        let file_ext = crate::fttype::get_file_extension("markdown");

        crate::tools::Tooling::Misspell.test_format_snippet(input, output, &file_ext);
    }
}
