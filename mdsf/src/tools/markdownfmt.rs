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

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Markdownfmt
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}
