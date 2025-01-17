///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_yew_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--edition");
    cmd.arg("2021");
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("yew-fmt")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_yew_fmt_args(cmd.build(), file_path);
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
mod test_yew_fmt {
    #[test_with::executable(yew-fmt)]
    fn test_yew_fmt_rust_70ad564760e773e9() {
        let input = r#"pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    "#;
        let output = r#"pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;
        let file_ext = crate::fttype::get_file_extension("rust");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::yew_fmt::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
