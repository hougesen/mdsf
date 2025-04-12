///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("roc")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_roc_format {
    #[test_with::executable(roc)]
    fn test_roc_format_roc_1204aa2d8186919d() {
        let input = r#"app "helloWorld"
    packages { pf: "https://github.com/roc-lang/" }
    imports [pf.Stdout]
    provides [main] to pf






main =
    Stdout.line "Hello, World!"


    "#;

        let output = r#"app [main] { pf: platform "https://github.com/roc-lang/" }

import pf.Stdout

main =
    Stdout.line "Hello, World!"

"#;

        let file_ext = crate::fttype::get_file_extension("roc");

        crate::tools::Tooling::RocFormat.test_format_snippet(input, output, &file_ext);
    }
}
