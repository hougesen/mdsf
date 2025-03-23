///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("jsonlint"),
    CommandType::Direct("jsonlint"),
    CommandType::Npm("jsonlint"),
    CommandType::Pnpm("jsonlint"),
    CommandType::Bun("jsonlint"),
    CommandType::Deno("jsonlint"),
    CommandType::Yarn("jsonlint"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_jsonlint {
    #[test_with::executable(jsonlint || npx || pnpm || deno || bunx)]
    fn test_jsonlint_json_5d1a6be238b35a5c() {
        let input = r#"{ "k": "v" }"#;

        let output = r#"{
  "k": "v"
}"#;

        let file_ext = crate::fttype::get_file_extension("json");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Jsonlint
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
