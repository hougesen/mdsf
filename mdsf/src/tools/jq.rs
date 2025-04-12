///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(cmd: std::process::Command, _file_path: &std::path::Path) -> std::process::Command {
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("jq")];

pub const IS_STDIN: bool = true;

#[cfg(test)]
mod test_jq {
    #[test_with::executable(jq)]
    fn test_jq_json_fddcd253f4dfd781() {
        let input = r#"{"key":1}"#;

        let output = r#"{
  "key": 1
}
"#;

        let file_ext = crate::fttype::get_file_extension("json");

        crate::tools::Tooling::Jq.test_format_snippet(input, output, &file_ext);
    }
}
