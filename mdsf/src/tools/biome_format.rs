///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_biome_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("biome"),
        CommandType::Direct("biome"),
        CommandType::Npm("@biomejs/biome"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_biome_format_args(cmd.build(), file_path);
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
mod test_biome_format {
    #[test_with::executable(npx)]
    fn test_biome_format_json_3bf561a65ea19c27() {
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
        let result = crate::tools::biome_format::run(snippet.path())
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(npx)]
    fn test_biome_format_javascript_8b78b3bf4549dcd5() {
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
        let result = crate::tools::biome_format::run(snippet.path())
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(npx)]
    fn test_biome_format_typescript_4c7d2a2ce681e640() {
        let input = r#"
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
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
        let result = crate::tools::biome_format::run(snippet.path())
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
