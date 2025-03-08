///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--embedded-language-formatting");
    cmd.arg("off");
    cmd.arg("--log-level");
    cmd.arg("error");
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 6] = [
    CommandType::NodeModules("prettier"),
    CommandType::Direct("prettier"),
    CommandType::Npm("prettier"),
    CommandType::Pnpm("prettier"),
    CommandType::Bun("prettier"),
    CommandType::Deno("prettier"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_prettier {
    #[test_with::executable(prettier || npx || pnpm || deno || bunx)]
    fn test_prettier_json_8e1e8ed2224fd439() {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
  "key": "value",
  "key2": ["value2", "value3", 1, null]
}
"#;

        let file_ext = crate::fttype::get_file_extension("json");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Prettier
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

    #[test_with::executable(prettier || npx || pnpm || deno || bunx)]
    fn test_prettier_javascript_f38217e7df306e3e() {
        let input = r#"
    async function asyncAddition(
            a,b
        ) {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a, b) {
  return a + b;
}
"#;

        let file_ext = crate::fttype::get_file_extension("javascript");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Prettier
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
