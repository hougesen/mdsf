///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_elm_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--elm-version=0.19");
    cmd.arg("--yes");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("elm-format"),
        CommandType::Direct("elm-format"),
        CommandType::Npm("elm-format"),
    ];

    crate::execution::run_tools(&commands, file_path, timeout, set_elm_format_args)
}

#[cfg(test)]
mod test_elm_format {
    #[test_with::executable(npx)]
    fn test_elm_format_elm_41c70725416bd82a() {
        let input = r#"import   Html       exposing   (text)


main =
      text              "Hello!"


  "#;
        let output = Some(
            r#"module Main exposing (main)

import Html exposing (text)


main =
    text "Hello!"
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("elm");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::elm_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
