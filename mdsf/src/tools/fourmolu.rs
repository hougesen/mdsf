use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_fourmolu_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("fourmolu")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_fourmolu_args(cmd.build(), file_path);
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
mod test_fourmolu {
    #[test_with::executable(fourmolu)]
    fn test_fourmolu_haskell_718612a8aa064d19() {
        let input = r#"
addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;
        let output = r#"addNumbers :: Int -> Int -> Int
addNumbers a b = do
    a + b
"#;
        let file_ext = crate::fttype::get_file_extension("haskell");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::fourmolu::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
