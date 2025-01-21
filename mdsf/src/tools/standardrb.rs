///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_standardrb_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("standardrb")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_standardrb_args(cmd.build(), file_path);
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
mod test_standardrb {
    #[test_with::executable(standardrb)]
    fn test_standardrb_ruby_fa74023fdfeb57a9() {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;
        let output = Some(
            r#"def add(a, b)
  a + b
end
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("ruby");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::standardrb::run(snippet.path())
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
