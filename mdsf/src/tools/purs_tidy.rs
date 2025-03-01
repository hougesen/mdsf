///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format-in-place");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("purs-tidy"),
    CommandType::Direct("purs-tidy"),
    CommandType::Npm("purs-tidy"),
];

#[cfg(test)]
mod test_purs_tidy {
    const TIMEOUT: u64 = 0;
    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(npx)]
    fn test_purs_tidy_purescript_c9e6831b630f7f08() {
        let input = r#"module       Test.Main   where

import Prelude

import   Effect   (Effect)
import                  Effect.Class.Console  (log)

main     ::   Effect Unit
main   =    do
  log          "You should add some tests.""#;

        let output = r#"module Test.Main where

import Prelude

import Effect (Effect)
import Effect.Class.Console (log)

main :: Effect Unit
main = do
  log "You should add some tests.""#;

        let file_ext = crate::fttype::get_file_extension("purescript");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            false,
            DEBUG_ENABLED,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
