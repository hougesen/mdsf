use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_standardjs_args(cmd: &mut tokio::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--fix").arg(snippet_path);
}

#[inline]
async fn invoke_standardjs(
    mut cmd: tokio::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_standardjs_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_standardjs(tokio::process::Command::new("standard"), snippet_path).await
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_standardjs(setup_npm_script("standard"), snippet_path).await
}

#[cfg(test)]
mod test_standardjs {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(standard)]
    async fn it_should_format_javascript() {
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

        let snippet = setup_snippet(input, language_to_ext("javascript"))
            .await
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
