///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_markdownlint_cli_2_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("markdownlint-cli2"),
        CommandType::Direct("markdownlint-cli2"),
        CommandType::Npm("markdownlint-cli2"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_markdownlint_cli_2_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path);

        if index == commands.len() - 1 {
            return execution_result;
        }

        if let Ok(r) = execution_result {
            if !r.0 {
                return Ok(r);
            }
        }
    }

    Ok((true, None))
}

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
        let result = crate::tools::markdownlint_cli_2::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
