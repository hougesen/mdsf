///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_grain_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd.arg("-o");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("grain")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_grain_format_args(cmd.build(), file_path);
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
mod test_grain_format {
    #[test_with::executable(grain)]
    fn test_grain_format_grain_58d469af4df1abea() {
        let input = r#"module Hello

                                print("Hello, world!")
"#;
        let output = Some(
            r#"module Hello

print("Hello, world!")
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("grain");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::grain_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
