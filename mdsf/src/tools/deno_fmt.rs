///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_deno_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("fmt");
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("deno")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_deno_fmt_args(cmd.build(), file_path);
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
mod test_deno_fmt {
    #[test_with::executable(deno)]
    fn test_deno_fmt_javascript_d7445fa122fcd5cc() {
        let input = r#"
    async function asyncAddition(a,b){
        return a+b
    }

            "#;
        let output = r#"async function asyncAddition(a, b) {
  return a + b;
}
"#;
        let file_ext = crate::fttype::get_file_extension("javascript");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::deno_fmt::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }

    #[test_with::executable(deno)]
    fn test_deno_fmt_typescript_857476c85438ce71() {
        let input = r#"
    async function asyncAddition(                                a:       	number,b:number ) :Promise< number>
    {
        return a+b
    }

            "#;
        let output = r#"async function asyncAddition(a: number, b: number): Promise<number> {
  return a + b;
}
"#;
        let file_ext = crate::fttype::get_file_extension("typescript");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::deno_fmt::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }

    #[test_with::executable(deno)]
    fn test_deno_fmt_json_d426a9ade74002d2() {
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
        let result = crate::tools::deno_fmt::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
