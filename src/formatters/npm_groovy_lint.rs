use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_npm_groovy_lint_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--format");
    cmd.arg(snippet_path);
}

#[inline]
fn invoke_npm_groovy_lint(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_npm_groovy_lint_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn format_using_npm_groovy_lint(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_npm_groovy_lint(std::process::Command::new("npm-groovy-lint"), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_npm_groovy_lint(setup_npm_script("npm-groovy-lint"), snippet_path)
}

#[cfg(test)]
mod test_npm_groovy_lint {
    use crate::{
        formatters::{npm_groovy_lint::format_using_npm_groovy_lint, setup_snippet},
        languages::Language,
    };

    #[test]
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

        let snippet = setup_snippet(input, Language::Groovy.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_npm_groovy_lint(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
