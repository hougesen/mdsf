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
    CommandType::NodeModules("markdownlint-cli2"),
    CommandType::Direct("markdownlint-cli2"),
    CommandType::Npm("markdownlint-cli2"),
    CommandType::Pnpm("markdownlint-cli2"),
    CommandType::Bun("markdownlint-cli2"),
    CommandType::Deno("markdownlint-cli2"),
    CommandType::Yarn("markdownlint-cli2"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_markdownlint_cli_2 {
    #[test_with::executable(markdownlint-cli2 || npx || pnpm || deno || bunx)]
    fn test_markdownlint_cli_2_markdown_1f615768d8e575c5() {
        let input = r#"# Hello world

- asd 
* vasd
"#;

        let output = r#"# Hello world

- asd
- vasd
"#;

        let file_ext = crate::fttype::get_file_extension("markdown");

        crate::tools::Tooling::MarkdownlintCli2.test_format_snippet(input, output, &file_ext);
    }
}
