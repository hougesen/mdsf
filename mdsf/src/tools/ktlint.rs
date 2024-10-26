use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_ktlint_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--format");
    cmd.arg("--log-level=error");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("ktlint")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_ktlint_args(cmd.build(), file_path);
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
mod test_ktlint {
    #[test_with::executable(ktlint)]
    fn test_ktlint_kotlin_a260d95cd982ccac() {
        let input = r#"            fun add(a:Int ,b:Int ):Int {
                    return a + b
                }
            "#;
        let output = r#"

fun add(
    a: Int,
    b: Int,
): Int {
    return a + b
}
"#;
        let file_ext = crate::fttype::get_file_extension("kotlin");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::ktlint::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}