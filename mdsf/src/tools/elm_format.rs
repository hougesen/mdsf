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

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("elm-format"),
    CommandType::Direct("elm-format"),
    CommandType::Npm("elm-format"),
    CommandType::Pnpm("elm-format"),
    CommandType::Bun("elm-format"),
    CommandType::Deno("elm-format"),
    CommandType::Yarn("elm-format"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_elm_format {
    #[test_with::executable(elm-format || npx || pnpm || deno || bunx)]
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

        crate::tools::Tooling::ElmFormat.test_format_snippet(input, output, &file_ext);
    }
}
