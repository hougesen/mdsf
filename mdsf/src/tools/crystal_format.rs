///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_crystal_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("tool");
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("crystal")];

    crate::execution::run_tools(&commands, file_path, timeout, set_crystal_format_args)
}

#[cfg(test)]
mod test_crystal_format {
    #[test_with::executable(crystal)]
    fn test_crystal_format_crystal_2cc833585f9c0931() {
        let input = r#"def add(a, b)  return a + b end"#;
        let output = Some(
            r#"def add(a, b)
  return a + b
end
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("crystal");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::crystal_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
