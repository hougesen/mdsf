use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_elm_format_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--elm-version=0.19").arg("--yes").arg(snippet_path);
}

#[inline]
fn invoke_elm_format(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_elm_format_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
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
    use crate::{
        formatters::{elm_format::run, setup_snippet},
        generated::language_to_ext,
    };

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
            setup_snippet(input, &language_to_ext("elm")).expect("it to create a snippet file");

        let output = run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
