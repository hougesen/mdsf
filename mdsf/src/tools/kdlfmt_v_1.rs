///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg("--kdl-version");
    cmd.arg("v1");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("kdlfmt"),
    CommandType::Direct("kdlfmt"),
    CommandType::Npm("kdlfmt"),
    CommandType::Pnpm("kdlfmt"),
    CommandType::Bun("kdlfmt"),
    CommandType::Deno("kdlfmt"),
    CommandType::Yarn("kdlfmt"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_kdlfmt_v_1 {
    #[test_with::executable(kdlfmt || npx || pnpm || deno || bunx)]
    fn test_kdlfmt_v_1_kdl_4324893eeee4a998() {
        let input = r#"world {
    child "1"
child "2"
     }
"#;

        let output = r#"world {
    child "1"
    child "2"
}
"#;

        let file_ext = crate::fttype::get_file_extension("kdl");

        crate::tools::Tooling::KdlfmtV1.test_format_snippet(input, output, &file_ext);
    }
}
