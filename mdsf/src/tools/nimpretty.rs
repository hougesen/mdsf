///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_nimpretty_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("nimpretty")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_nimpretty_args(cmd.build(), file_path);
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
mod test_nimpretty {
    #[test_with::executable(nimpretty)]
    fn test_nimpretty_nim_2c41c79e1d74972a() {
        let input = r#"proc           add( a         :int , b:int )        : int =
  return a +          b  "#;
        let output = r#"proc add(a: int, b: int): int =
  return a + b
"#;
        let file_ext = crate::fttype::get_file_extension("nim");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::nimpretty::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
