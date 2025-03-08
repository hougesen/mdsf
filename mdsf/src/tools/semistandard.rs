///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg("--stdin");
    cmd
}

pub const COMMANDS: [CommandType; 6] = [
    CommandType::NodeModules("semistandard"),
    CommandType::Direct("semistandard"),
    CommandType::Npm("semistandard"),
    CommandType::Pnpm("semistandard"),
    CommandType::Bun("semistandard"),
    CommandType::Deno("semistandard"),
];

pub const IS_STDIN: bool = true;

#[cfg(test)]
mod test_semistandard {
    #[test_with::executable(npx)]
    fn test_semistandard_javascript_dd13bf6b8d6e09a1() {
        let input = r#"    async function asyncAddition(a,b  )
    {
        return a+b
    }

console.info(asyncAddition(1, 2));
            "#;

        let output = r#"async function asyncAddition (a, b) {
  return a + b;
}

console.info(asyncAddition(1, 2));
"#;

        let file_ext = crate::fttype::get_file_extension("javascript");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Semistandard
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
