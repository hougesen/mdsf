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

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("purs-tidy"),
    CommandType::Direct("purs-tidy"),
    CommandType::Npm("purs-tidy"),
    CommandType::Pnpm("purs-tidy"),
    CommandType::Bun("purs-tidy"),
    CommandType::Deno("purs-tidy"),
    CommandType::Yarn("purs-tidy"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_purs_tidy {
    #[test_with::executable(purs-tidy || npx || pnpm || deno || bunx)]
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

        let result = crate::tools::Tooling::PursTidy
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}
