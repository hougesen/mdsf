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

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("standard"),
    CommandType::Direct("standard"),
    CommandType::Npm("standard"),
    CommandType::Pnpm("standard"),
    CommandType::Bun("standard"),
    CommandType::Deno("standard"),
    CommandType::Yarn("standard"),
];

pub const IS_STDIN: bool = true;

#[cfg(test)]
mod test_standardjs {
    #[test_with::executable(standard || npx || pnpm || deno || bunx)]
    fn test_standardjs_javascript_548a80949cde541f() {
        let input = r#"
    async function asyncAddition(a,b  )
    {
        return a+b
    }

console.info(asyncAddition(1, 2));
            "#;

        let output = r#"async function asyncAddition (a, b) {
  return a + b
}

console.info(asyncAddition(1, 2))
"#;

        let file_ext = crate::fttype::get_file_extension("javascript");

        crate::tools::Tooling::Standardjs.test_format_snippet(input, output, &file_ext);
    }
}
