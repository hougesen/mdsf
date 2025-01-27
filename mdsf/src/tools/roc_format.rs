///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_roc_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("roc")];

    crate::execution::run_tools(&commands, file_path, timeout, set_roc_format_args)
}

#[cfg(test)]
mod test_roc_format {
    #[test_with::executable(roc)]
    fn test_roc_format_roc_8201b2f4943b5b12() {
        let input = r#"app "helloWorld"
    packages { pf: "https://github.com/roc-lang/" }
    imports [pf.Stdout]
    provides [main] to pf






main =
    Stdout.line "Hello, World!"


    "#;
        let output = Some(
            r#"app [main] { pf: platform "https://github.com/roc-lang/" }

import pf.Stdout

main =
    Stdout.line "Hello, World!"

"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("roc");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::roc_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
