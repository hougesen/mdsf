///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("oxlint"),
    CommandType::Direct("oxlint"),
    CommandType::Npm("oxlint"),
    CommandType::Pnpm("oxlint"),
    CommandType::Bun("oxlint"),
    CommandType::Deno("oxlint"),
    CommandType::Yarn("oxlint"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_oxlint {
    #[test_with::executable(oxlint || npx || pnpm || deno || bunx)]
    fn test_oxlint_typescript_a2154a11ef1c153b() {
        let input = r#"debugger;"#;

        let output = r#""#;

        let file_ext = crate::fttype::get_file_extension("typescript");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Oxlint
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
