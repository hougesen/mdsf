///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_mado_check_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("check");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("mado")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_mado_check_args(cmd.build(), file_path);
        let execution_result =
            execute_command(cmd, file_path, timeout).map(|value| (value.0, None));

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
mod test_mado_check {
    #[test_with::executable(mado)]
    fn test_mado_check_markdown_dec3c84890502a43() {
        let input = r#"# Hello world

- Hello
- world

"#;
        let output = None;
        let file_ext = crate::fttype::get_file_extension("markdown");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::mado_check::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
