///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("markdownlint"),
    CommandType::Direct("markdownlint"),
    CommandType::Npm("markdownlint-cli"),
    CommandType::Pnpm("markdownlint-cli"),
    CommandType::Bun("markdownlint-cli"),
    CommandType::Deno("markdownlint-cli"),
    CommandType::Yarn("markdownlint-cli"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_markdownlint {
    #[test_with::executable(markdownlint || npx || pnpm || deno || bunx)]
    fn test_markdownlint_markdown_27f5778fc1db5182() {
        let input = r#"# Hello world

- asd
* vasd
"#;

        let output = r#"# Hello world

- asd
- vasd
"#;

        let file_ext = crate::fttype::get_file_extension("markdown");

        crate::tools::Tooling::Markdownlint.test_format_snippet(input, output, &file_ext);
    }
}
