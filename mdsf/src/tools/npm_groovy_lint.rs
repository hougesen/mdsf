use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_npm_groovy_lint_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("npm-groovy-lint"),
        CommandType::Direct("npm-groovy-lint"),
        CommandType::Npm("npm-groovy-lint"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_npm_groovy_lint_args(cmd.build(), file_path);
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
mod test_npm_groovy_lint {
    #[test_with::executable(npx)]
    fn test_npm_groovy_lint_groovy_9484173186968a23() {
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
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::npm_groovy_lint::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
