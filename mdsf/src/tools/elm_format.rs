///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_elm_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--elm-version=0.19");
    cmd.arg("--yes");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("elm-format"),
        CommandType::Direct("elm-format"),
        CommandType::Npm("elm-format"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_elm_format_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path);

        if index == commands.len() - 1 {
            return execution_result;
        }

        if let Ok(r) = execution_result {
            if !r.0 {
                return Ok(r);
            }
        }
    }

    Ok((true, None))
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
        let result = crate::tools::elm_format::run(snippet.path())
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
