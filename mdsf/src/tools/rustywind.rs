///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("rustywind"),
    CommandType::Direct("rustywind"),
    CommandType::Npm("rustywind"),
    CommandType::Pnpm("rustywind"),
    CommandType::Bun("rustywind"),
    CommandType::Deno("rustywind"),
    CommandType::Yarn("rustywind"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_rustywind {
    #[test_with::executable(rustywind || npx || pnpm || deno || bunx)]
    fn test_rustywind_html_f482eb2ece82bb0d() {
        let input = r#"<div class="flex-col flex"></div>
"#;

        let output = r#"<div class="flex flex-col"></div>
"#;

        let file_ext = crate::fttype::get_file_extension("html");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Rustywind
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
