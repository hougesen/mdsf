///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_deno_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("fmt");
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("deno")];

    crate::execution::run_tools(&commands, file_path, timeout, set_deno_fmt_args)
}

#[cfg(test)]
mod test_deno_fmt {
    #[test_with::executable(deno)]
    fn test_deno_fmt_javascript_7a64a04e758eecef() {
        let input = r#"
    async function asyncAddition(a,b){
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
        let result = crate::tools::deno_fmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(deno)]
    fn test_deno_fmt_typescript_7f6a42002496954d() {
        let input = r#"
    async function asyncAddition(                                a:       	number,b:number ) :Promise< number>
    {
        return a+b
    }

            "#;
        let output = Some(
            r#"async function asyncAddition(a: number, b: number): Promise<number> {
  return a + b;
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("typescript");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::deno_fmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(deno)]
    fn test_deno_fmt_json_9cf77f718b7f7e0c() {
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
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("json");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::deno_fmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
