///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_scalafmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--quiet");
    cmd.arg("--mode");
    cmd.arg("any");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("scalafmt")];

    crate::execution::run_tools(&commands, file_path, timeout, set_scalafmt_args)
}

#[cfg(test)]
mod test_scalafmt {
    #[test_with::executable(scalafmt)]
    fn test_scalafmt_scala_3a9ebb0c3854e5cc() {
        let input = r#"object Addition {
             def main() = {
                 println(1 + 3)
             }
    }"#;
        let output = Some(
            r#"object Addition {
  def main() = {
    println(1 + 3)
  }
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("scala");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::scalafmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
