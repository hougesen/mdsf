///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
fn set_markdownlint_cli_2_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("markdownlint-cli2"),
    CommandType::Direct("markdownlint-cli2"),
    CommandType::Npm("markdownlint-cli2"),
];

#[inline]
pub fn run(
    file_path: &std::path::Path,
    timeout: u64,
) -> Result<(bool, Option<String>), crate::error::MdsfError> {
    crate::execution::run_tools(&COMMANDS, file_path, timeout, set_markdownlint_cli_2_args)
}

#[cfg(test)]
mod test_markdownlint_cli_2 {
    #[test_with::executable(npx)]
    fn test_markdownlint_cli_2_markdown_40f9776569512cd() {
        let input = r#"# Hello world

- asd 
* vasd
"#;
        let output = Some(
            r#"# Hello world

- asd
- vasd
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("markdown");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::markdownlint_cli_2::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
