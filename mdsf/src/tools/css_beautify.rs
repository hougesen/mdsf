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
    cmd.arg("--type");
    cmd.arg("css");
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("css-beautify"),
    CommandType::Direct("css-beautify"),
    CommandType::Npm("js-beautify"),
    CommandType::Pnpm("js-beautify"),
    CommandType::Bun("js-beautify"),
    CommandType::Deno("js-beautify"),
    CommandType::Yarn("js-beautify"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_css_beautify {
    #[test_with::executable(css-beautify || npx || pnpm || deno || bunx)]
    fn test_css_beautify_css_5ad41f26f69aea3e() {
        let input = r#"h1   {color: blue;} p    {color: red;}"#;

        let output = r#"h1 {
    color: blue;
}

p {
    color: red;
}"#;

        let file_ext = crate::fttype::get_file_extension("css");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::CssBeautify
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
