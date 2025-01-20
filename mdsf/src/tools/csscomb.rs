///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_csscomb_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-t");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("csscomb"),
        CommandType::Direct("csscomb"),
        CommandType::Npm("csscomb"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_csscomb_args(cmd.build(), file_path);
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
mod test_csscomb {
    #[test_with::executable(npx)]
    fn test_csscomb_css_bed67a883a4a1aae() {
        let input = r#"h1   {color: blue;}
p {color: red;}"#;
        let output = r#"h1
{
    color: blue;
}
p
{
    color: red;
}
"#;
        let file_ext = crate::fttype::get_file_extension("css");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::csscomb::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
