///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_auto_optional_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("auto-optional")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_auto_optional_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path, timeout);

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
mod test_auto_optional {
    #[test_with::executable(auto-optional)]
    fn test_auto_optional_python_d5dd242171892fcc() {
        let input = r#"def foo(bar: str = None):
    pass
"#;
        let output = Some(
            r#"from typing import Optional
def foo(bar: Optional[str] = None):
    pass
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("python");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::auto_optional::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
