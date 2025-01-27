///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_prettier_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--embedded-language-formatting");
    cmd.arg("off");
    cmd.arg("--log-level");
    cmd.arg("error");
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("prettier"),
        CommandType::Direct("prettier"),
        CommandType::Npm("prettier"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_prettier_args(cmd.build(), file_path);
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
mod test_prettier {
    #[test_with::executable(npx)]
    fn test_prettier_json_3694f4bf312c36fa() {
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
        let output = Some(
            r#"{
  "key": "value",
  "key2": ["value2", "value3", 1, null]
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("json");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::prettier::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(npx)]
    fn test_prettier_javascript_e4047d8692b0d84a() {
        let input = r#"
    async function asyncAddition(
            a,b
        ) {
        return a+b
    }

            "#;
        let output = Some(
            r#"async function asyncAddition(a, b) {
  return a + b;
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("javascript");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::prettier::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
