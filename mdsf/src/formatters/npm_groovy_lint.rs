use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_npm_groovy_lint_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--format").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_npm_groovy_lint(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_npm_groovy_lint_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_npm_groovy_lint(
        CommandType::NodeModules("npm-groovy-lint").build(),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) =
        invoke_npm_groovy_lint(CommandType::Direct("npm-groovy-lint").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_npm_groovy_lint(CommandType::Npm("npm-groovy-lint").build(), snippet_path)
}

#[cfg(test)]
mod test_npm_groovy_lint {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(npx)]
    fn it_should_format_groovy() {
        let input = "                  def add(a, b) {
            return a + b
        }

        assert add(1,2) == 3 ";

        let expected_output = "def add(a, b) {
    return a + b
}

assert add(1, 2) == 3
";

        let snippet = setup_snippet(input, &get_file_extension("groovy"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
