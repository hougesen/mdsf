///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--format");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("npm-groovy-lint"),
    CommandType::Direct("npm-groovy-lint"),
    CommandType::Npm("npm-groovy-lint"),
    CommandType::Pnpm("npm-groovy-lint"),
    CommandType::Bun("npm-groovy-lint"),
    CommandType::Deno("npm-groovy-lint"),
    CommandType::Yarn("npm-groovy-lint"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_npm_groovy_lint {
    #[test_with::executable(npm-groovy-lint || npx || pnpm || deno || bunx)]
    fn test_npm_groovy_lint_groovy_2dc2be09d8013576() {
        let input = r#"                  def add(a, b) {
            return a + b
        }

        assert add(1,2) == 3 "#;

        let output = r#"def add(a, b) {
    return a + b
}

assert add(1, 2) == 3
"#;

        let file_ext = crate::fttype::get_file_extension("groovy");

        crate::tools::Tooling::NpmGroovyLint.test_format_snippet(input, output, &file_ext);
    }
}
