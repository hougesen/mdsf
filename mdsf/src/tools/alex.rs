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

pub const COMMANDS: [CommandType; 6] = [
    CommandType::NodeModules("alex"),
    CommandType::Direct("alex"),
    CommandType::Npm("alex"),
    CommandType::Pnpm("alex"),
    CommandType::Bun("alex"),
    CommandType::Deno("alex"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_alex {
    #[test_with::executable(alex || npx || pnpm || deno || bunx)]
    fn test_alex_markdown_114ca1bc58b35aef() {
        let input = r#"hello"#;

        let output = r#"hello"#;

        let file_ext = crate::fttype::get_file_extension("markdown");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Alex
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
