use super::execute_command;
use crate::{runners::setup_npm_script, terminal::print_formatter_info};

#[inline]
fn set_elm_format_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--elm-version=0.19").arg("--yes").arg(snippet_path);
}

#[inline]
fn invoke_elm_format(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    set_elm_format_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn format_using_elm_format(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("elm_format");

    invoke_elm_format(setup_npm_script("elm-format"), snippet_path)
}

#[cfg(test)]
mod test_elm_format {
    use crate::{
        formatters::{elm_format::format_using_elm_format, setup_snippet},
        languages::Language,
    };

    #[test]
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
            setup_snippet(input, Language::Elm.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_elm_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
