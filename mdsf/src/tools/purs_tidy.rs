///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_purs_tidy_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format-in-place");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("purs-tidy"),
        CommandType::Direct("purs-tidy"),
        CommandType::Npm("purs-tidy"),
    ];

    crate::execution::run_tools(&commands, file_path, timeout, set_purs_tidy_args)
}

#[cfg(test)]
mod test_purs_tidy {
    #[test_with::executable(npx)]
    fn test_purs_tidy_purescript_9d25e0f9177f47be() {
        let input = r#"module       Test.Main   where

import Prelude

import   Effect   (Effect)
import                  Effect.Class.Console  (log)

main     ::   Effect Unit
main   =    do
  log          "You should add some tests.""#;
        let output = Some(
            r#"module Test.Main where

import Prelude

import Effect (Effect)
import Effect.Class.Console (log)

main :: Effect Unit
main = do
  log "You should add some tests.""#
                .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("purescript");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::purs_tidy::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
