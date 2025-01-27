///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_ocp_indent_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--inplace");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("ocp-indent")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_ocp_indent_args(cmd.build(), file_path);
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
mod test_ocp_indent {
    #[test_with::executable(ocp-indent)]
    fn test_ocp_indent_ocaml_10b8e4c4674bf81c() {
        let input = r#"
let add a b
                             = a + b
            "#;
        let output = Some(
            r#"
let add a b
  = a + b
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("ocaml");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::ocp_indent::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
