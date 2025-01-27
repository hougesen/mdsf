///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_mix_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("mix")];

    crate::execution::run_tools(&commands, file_path, timeout, set_mix_format_args)
}

#[cfg(test)]
mod test_mix_format {
    #[test_with::executable(mix)]
    fn test_mix_format_elixir_bd36c92a13ed14e6() {
        let input = r#"
        def              add(a  ,      b   )   do    a   +   b                 end

"#;
        let output = Some(
            r#"def add(a, b) do
  a + b
end
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("elixir");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::mix_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
