///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("verify");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("mdsf"),
    CommandType::Direct("mdsf"),
    CommandType::Npm("mdsf-cli"),
    CommandType::Pnpm("mdsf-cli"),
    CommandType::Bun("mdsf-cli"),
    CommandType::Deno("mdsf-cli"),
    CommandType::Yarn("mdsf-cli"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_mdsf_verify {
    #[test_with::executable(mdsf || npx || pnpm || deno || bunx)]
    fn test_mdsf_verify_markdown_1e1586f943958589() {
        let input = r#""#;

        let output = r#""#;

        let file_ext = crate::fttype::get_file_extension("markdown");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::MdsfVerify
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
