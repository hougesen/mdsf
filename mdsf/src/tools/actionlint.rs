///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_actionlint_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("actionlint")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_actionlint_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path).map(|value| (value.0, None));

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
mod test_actionlint {
    #[test_with::executable(actionlint)]
    fn test_actionlint_yaml_e8ea2c4c1494f1e5() {
        let input = r#"name: action
on: push
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;
        let output = None;
        let file_ext = crate::fttype::get_file_extension("yaml");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::actionlint::run(snippet.path())
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
