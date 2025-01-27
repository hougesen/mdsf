///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_arguments(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("markdownlint-cli2"),
    CommandType::Direct("markdownlint-cli2"),
    CommandType::Npm("markdownlint-cli2"),
];

#[cfg(test)]
mod test_markdownlint_cli_2 {
    #[test_with::executable(npx)]
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

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result =
            crate::execution::run_tools(&super::COMMANDS, snippet.path(), super::set_arguments, 0)
                .expect("it to be successful")
                .1
                .expect("it to be some");

        assert_eq!(result, output);
    }
}
