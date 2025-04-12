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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("markdownfmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_markdownfmt {
    #[test_with::executable(markdownfmt)]
    fn test_markdownfmt_markdown_9b495bc15a7833bc() {
        let input = r#"# hello w   world

this   text has      weird spacing

- first
* second"#;

        let output = r#"hello w world
=============

this text has weird spacing

-	first
-	second
"#;

        let file_ext = crate::fttype::get_file_extension("markdown");

        crate::tools::Tooling::Markdownfmt.test_format_snippet(input, output, &file_ext);
    }
}
