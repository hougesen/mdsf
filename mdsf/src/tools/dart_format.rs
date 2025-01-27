///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_dart_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("dart")];

    crate::execution::run_tools(&commands, file_path, timeout, set_dart_format_args)
}

#[cfg(test)]
mod test_dart_format {
    #[test_with::executable(dart)]
    fn test_dart_format_dart_d00f785063b3cede() {
        let input = r#"class Adder {   int add(int a, int b) {     return a + b;   } }    "#;
        let output = Some(
            r#"class Adder {
  int add(int a, int b) {
    return a + b;
  }
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("dart");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::dart_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
