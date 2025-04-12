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
    CommandType::NodeModules("semistandard"),
    CommandType::Direct("semistandard"),
    CommandType::Npm("semistandard"),
    CommandType::Pnpm("semistandard"),
    CommandType::Bun("semistandard"),
    CommandType::Deno("semistandard"),
    CommandType::Yarn("semistandard"),
];

pub const IS_STDIN: bool = true;

#[cfg(test)]
mod test_semistandard {
    #[test_with::executable(semistandard || npx || pnpm || deno || bunx)]
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

        crate::tools::Tooling::Semistandard.test_format_snippet(input, output, &file_ext);
    }
}
