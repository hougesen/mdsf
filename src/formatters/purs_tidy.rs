use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_purs_tidy_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format-in-place").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_purs_tidy(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_purs_tidy_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_purs_tidy(CommandType::NodeModules("purs-tidy").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) =
        invoke_purs_tidy(CommandType::Direct("purs-tidy").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_purs_tidy(CommandType::Npm("purs-tidy").build(), snippet_path)
}

#[cfg(test)]
mod test_purs_tidy {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(npx)]
    fn it_should_format_purescript() {
        let input = r#"module       Test.Main   where

import Prelude

import   Effect   (Effect)
import                  Effect.Class.Console  (log)

main     ::   Effect Unit
main   =    do
  log          "You should add some tests.""#;

        let expected_output = r#"module Test.Main where

import Prelude

import Effect (Effect)
import Effect.Class.Console (log)

main :: Effect Unit
main = do
  log "You should add some tests.""#;

        let snippet = setup_snippet(input, language_to_ext("purescript"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
