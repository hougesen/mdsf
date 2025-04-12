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

        crate::tools::Tooling::Jsonpp.test_format_snippet(input, output, &file_ext);
    }
}
