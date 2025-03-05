///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--elm-version=0.19");
    cmd.arg("--yes");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("elm-format"),
    CommandType::Direct("elm-format"),
    CommandType::Npm("elm-format"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_elm_format {
    const TIMEOUT: u64 = 0;

    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(npx)]
    fn test_elm_format_elm_4e120501af0177c4() {
        let input = r#"import   Html       exposing   (text)


main =
      text              "Hello!"


  "#;

        let output = r#"module Main exposing (main)

import Html exposing (text)


main =
    text "Hello!"
"#;

        let file_ext = crate::fttype::get_file_extension("elm");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
