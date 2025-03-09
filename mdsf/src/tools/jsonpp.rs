///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    _file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-s");
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("jsonpp")];

pub const IS_STDIN: bool = true;

#[cfg(test)]
mod test_jsonpp {
    #[test_with::executable(jsonpp)]
    fn test_jsonpp_json_d19292d79f47b2c7() {
        let input = r#"{
              "key": "value",
  "key2": ["value2", "value3", 1            , null]
 }"#;

        let output = r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}"#;

        let file_ext = crate::fttype::get_file_extension("json");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Jsonpp
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
