///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_mix_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("mix")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_mix_format_args(cmd.build(), file_path);
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
mod test_mix_format {
    #[test_with::executable(mix)]
    fn test_mix_format_elixir_ab535c627dfb140() {
        let input = r#"
        def              add(a  ,      b   )   do    a   +   b                 end

"#;
        let output = r#"def add(a, b) do
  a + b
end
"#;
        let file_ext = crate::fttype::get_file_extension("elixir");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::mix_format::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
