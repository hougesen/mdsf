///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_yapf_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--in-place");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("yapf")];

    crate::execution::run_tools(&commands, file_path, timeout, set_yapf_args)
}

#[cfg(test)]
mod test_yapf {
    #[test_with::executable(yapf)]
    fn test_yapf_python_fb3e2d124e8bebbb() {
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
        let result = crate::tools::yapf::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
