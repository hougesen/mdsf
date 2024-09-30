use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

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
    if let Ok(path_result) =
        invoke_elm_format(CommandType::NodeModules("elm-format").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) =
        invoke_elm_format(CommandType::Direct("elm-format").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_elm_format(CommandType::Npm("elm-format").build(), snippet_path)
}

#[cfg(test)]
mod test_elm_format {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

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
            setup_snippet(input, &get_file_extension("elm")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
