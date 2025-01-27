///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_nimpretty_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("nimpretty")];

    crate::execution::run_tools(&commands, file_path, timeout, set_nimpretty_args)
}

#[cfg(test)]
mod test_nimpretty {
    #[test_with::executable(nimpretty)]
    fn test_nimpretty_nim_904d741f6111f585() {
        let input = r#"proc           add( a         :int , b:int )        : int =
  return a +          b  "#;
        let output = Some(
            r#"proc add(a: int, b: int): int =
  return a + b
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("nim");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::nimpretty::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
