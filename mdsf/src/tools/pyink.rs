///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_pyink_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("pyink")];

    crate::execution::run_tools(&commands, file_path, timeout, set_pyink_args)
}

#[cfg(test)]
mod test_pyink {
    #[test_with::executable(pyink)]
    fn test_pyink_python_fb3e2d124e8bebbb() {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;
        let output = Some(
            r#"def add(a: int, b: int) -> int:
    return a + b
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("python");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::pyink::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
