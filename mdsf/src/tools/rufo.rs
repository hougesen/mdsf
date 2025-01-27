///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_rufo_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--simple-exit");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("rufo")];

    crate::execution::run_tools(&commands, file_path, timeout, set_rufo_args)
}

#[cfg(test)]
mod test_rufo {
    #[test_with::executable(rufo)]
    fn test_rufo_ruby_abe6af1ec08931cd() {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;
        let output = Some(
            r#"def add(a, b)
  return a + b
end
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("ruby");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::rufo::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
