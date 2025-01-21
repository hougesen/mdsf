///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_swift_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--in-place");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("swift-format")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_swift_format_args(cmd.build(), file_path);
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
mod test_swift_format {
    #[test_with::executable(swift-format)]
    fn test_swift_format_swift_b40925120220f7cd() {
        let input = r#" func add(a:Int ,b:Int)->Int {
    return a + b
    }"#;
        let output = Some(
            r#"func add(a: Int, b: Int) -> Int {
    return a + b
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("swift");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::swift_format::run(snippet.path())
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
