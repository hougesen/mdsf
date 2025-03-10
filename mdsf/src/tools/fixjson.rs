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

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("fixjson"),
    CommandType::Direct("fixjson"),
    CommandType::Npm("fixjson"),
    CommandType::Pnpm("fixjson"),
    CommandType::Bun("fixjson"),
    CommandType::Deno("fixjson"),
    CommandType::Yarn("fixjson"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_fixjson {
    #[test_with::executable(fixjson || npx || pnpm || deno || bunx)]
    fn test_fixjson_json_115ca7a7d8b2cc2b() {
        let input = r#"{     "fixjson": "fixjson"  }"#;

        let output = r#"{
  "fixjson": "fixjson"
}
"#;

        let file_ext = crate::fttype::get_file_extension("json");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Fixjson
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
