///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_gleam_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("gleam")];

    crate::execution::run_tools(&commands, file_path, timeout, set_gleam_format_args)
}

#[cfg(test)]
mod test_gleam_format {
    #[test_with::executable(gleam)]
    fn test_gleam_format_gleam_1c8414a45f66b1da() {
        let input = r#"pub fn add(a:Int,b:Int)->Int{a+b}"#;
        let output = Some(
            r#"pub fn add(a: Int, b: Int) -> Int {
  a + b
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("gleam");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::gleam_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
