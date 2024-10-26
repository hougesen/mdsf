use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_zig_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("fmt");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("zig")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_zig_fmt_args(cmd.build(), file_path);
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
mod test_zig_fmt {
    #[test_with::executable(zig)]
    fn test_zig_fmt_zig_c392cb7c52bb9cc8() {
        let input = r#"
    fn     add   (a : i32    , b :   i32 )             i32 {
        return a + b ;

    }
    "#;
        let output = r#"fn add(a: i32, b: i32) i32 {
    return a + b;
}
"#;
        let file_ext = crate::fttype::get_file_extension("zig");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::zig_fmt::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
