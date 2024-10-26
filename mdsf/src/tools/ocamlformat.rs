use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_ocamlformat_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--ignore-invalid-option");
    cmd.arg("--inplace");
    cmd.arg("--enable-outside-detected-project");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("ocamlformat")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_ocamlformat_args(cmd.build(), file_path);
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
mod test_ocamlformat {
    #[test_with::executable(ocamlformat)]
    fn test_ocamlformat_ocaml_a4abcc4d5c3404c0() {
        let input = r#"
let add a b  =  a +  b
            "#;
        let output = r#"let add a b = a + b
"#;
        let file_ext = crate::fttype::get_file_extension("ocaml");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::ocamlformat::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
