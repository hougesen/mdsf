use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_ocp_indent_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--inplace");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("ocp-indent")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_ocp_indent_args(cmd.build(), file_path);
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
mod test_ocp_indent {
    #[test_with::executable(ocp-indent)]
    fn test_ocp_indent_ocaml_5904945f3cb9a254() {
        let input = r#"
let add a b
                             = a + b
            "#;
        let output = r#"
let add a b
  = a + b
"#;
        let file_ext = crate::fttype::get_file_extension("ocaml");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::ocp_indent::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}