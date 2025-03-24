///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--html");
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("xq")];

pub const IS_STDIN: bool = true;

#[cfg(test)]
mod test_xq_html {
    #[test_with::executable(xq)]
    fn test_xq_html_html_aa297be5c685b338() {
        let input = r#"<div> 
         </div>"#;

        let output = r#"<div>
</div>
"#;

        let file_ext = crate::fttype::get_file_extension("html");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::XqHtml
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
