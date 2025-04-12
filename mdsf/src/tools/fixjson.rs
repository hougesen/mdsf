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

        crate::tools::Tooling::Fixjson.test_format_snippet(input, output, &file_ext);
    }
}
