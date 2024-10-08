use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_standardjs_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_standardjs(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_standardjs_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_standardjs(CommandType::NodeModules("standard").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) =
        invoke_standardjs(CommandType::Direct("standard").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_standardjs(CommandType::Npm("standard").build(), snippet_path)
}

#[cfg(test)]
mod test_standardjs {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(standard)]
    fn it_should_format_javascript() {
        let input = "
    async function asyncAddition(a,b  )
    {
        return a+b
    }

console.info(asyncAddition(1, 2));
            ";

        let expected_output = "async function asyncAddition (a, b) {
  return a + b
}

console.info(asyncAddition(1, 2))
";

        let snippet = setup_snippet(input, &get_file_extension("javascript"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
