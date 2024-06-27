use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_elm_format_args(cmd: &mut tokio::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--elm-version=0.19").arg("--yes").arg(snippet_path);
}

#[inline]
async fn invoke_elm_format(
    mut cmd: tokio::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_elm_format_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_elm_format(tokio::process::Command::new("elm-format"), snippet_path).await
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_elm_format(setup_npm_script("elm-format"), snippet_path).await
}

#[cfg(test)]
mod test_elm_format {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(npx)]
    async fn it_should_format_elm() {
        let input = r#"import   Html       exposing   (text)


main =
      text              "Hello!"


  "#;
        let expected_output = r#"module Main exposing (main)

import Html exposing (text)


main =
    text "Hello!"
"#;

        let snippet = setup_snippet(input, language_to_ext("elm"))
            .await
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
