///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--inplace");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("yq")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_yq {
    #[test_with::executable(yq)]
    fn test_yq_json_b20bccf3f90b7945() {
        let input = r#"{ "yq": "yq"  }"#;

        let output = r#"{
  "yq": "yq"
}
"#;

        let file_ext = crate::fttype::get_file_extension("json");

        crate::tools::Tooling::Yq.test_format_snippet(input, output, &file_ext);
    }
}
