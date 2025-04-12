///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-q");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("coffeelint"),
    CommandType::Direct("coffeelint"),
    CommandType::Npm("@coffeelint/cli"),
    CommandType::Pnpm("@coffeelint/cli"),
    CommandType::Bun("@coffeelint/cli"),
    CommandType::Deno("@coffeelint/cli"),
    CommandType::Yarn("@coffeelint/cli"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_coffeelint {
    #[test_with::executable(coffeelint || npx || pnpm || deno || bunx)]
    fn test_coffeelint_coffeescript_7b620f6d6e2ab16d() {
        let input = r#"add = (a, b) -> a + b"#;

        let output = r#"add = (a, b) -> a + b"#;

        let file_ext = crate::fttype::get_file_extension("coffeescript");

        crate::tools::Tooling::Coffeelint.test_format_snippet(input, output, &file_ext);
    }
}
