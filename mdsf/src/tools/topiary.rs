use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_topiary_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("topiary")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_topiary_args(cmd.build(), file_path);
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
mod test_topiary {
    #[test_with::executable(topiary)]
    fn test_topiary_json_d426a9ade74002d2() {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;
        let output = r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}
"#;
        let file_ext = crate::fttype::get_file_extension("json");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::topiary::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
