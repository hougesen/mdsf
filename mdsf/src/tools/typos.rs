///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_typos_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-w");
    cmd.arg("--no-ignore");
    cmd.arg("--hidden");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("typos")];

    crate::execution::run_tools(&commands, file_path, timeout, set_typos_args)
}

#[cfg(test)]
mod test_typos {
    #[test_with::executable(typos)]
    fn test_typos_python_45f392211f741c27() {
        let input = r#"anouncement"#;
        let output = Some(r#"announcement"#.to_owned());
        let file_ext = crate::fttype::get_file_extension("python");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::typos::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
