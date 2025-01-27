///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_pycln_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--no-gitignore");
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("pycln")];

    crate::execution::run_tools(&commands, file_path, timeout, set_pycln_args)
}

#[cfg(test)]
mod test_pycln {
    #[test_with::executable(pycln)]
    fn test_pycln_python_303d0fea3e20b822() {
        let input = r#"import math"#;
        let output = Some(r#""#.to_owned());
        let file_ext = crate::fttype::get_file_extension("python");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::pycln::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
