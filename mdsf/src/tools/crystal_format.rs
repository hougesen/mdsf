use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_crystal_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("tool");
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("crystal")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_crystal_format_args(cmd.build(), file_path);
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
mod test_crystal_format {
    #[test_with::executable(crystal)]
    fn test_crystal_format_crystal_e0f2d532cd984bee() {
        let input = r#"def add(a, b)  return a + b end"#;
        let output = r#"def add(a, b)
  return a + b
end
"#;
        let file_ext = crate::fttype::get_file_extension("crystal");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::crystal_format::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
