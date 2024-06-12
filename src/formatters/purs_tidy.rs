use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_purs_tidy_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("format-in-place").arg(snippet_path);
}

#[inline]
fn invoke_purs_tidy(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_purs_tidy_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn format_using_purs_tidy(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    invoke_purs_tidy(setup_npm_script("purs-tidy"), snippet_path)
}

#[cfg(test)]
mod test_purs_tidy {
    use super::format_using_purs_tidy;
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test]
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

        let snippet = setup_snippet(input, &language_to_ext("purescript"))
            .expect("it to create a snippet file");

        let output = format_using_purs_tidy(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
