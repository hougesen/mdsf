use super::execute_command;
use crate::{
    error::MdsfError,
    runners::{run_executable_from_path, setup_npm_script},
};

#[inline]
fn set_elm_format_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--elm-version=0.19").arg("--yes").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_elm_format(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_elm_format_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_elm_format(
        run_executable_from_path("node_modules/.bin/elm-format"),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) =
        invoke_elm_format(std::process::Command::new("elm-format"), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_elm_format(setup_npm_script("elm-format"), snippet_path)
}

#[cfg(test)]
mod test_elm_format {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(npx)]
    fn it_should_format_elm() {
        let input = r#"import   Html       exposing   (text)


main =
      text              "Hello!"


  "#;
        let expected_output = r#"module Main exposing (main)

import Html exposing (text)


main =
    text "Hello!"
"#;

        let snippet =
            setup_snippet(input, language_to_ext("elm")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
