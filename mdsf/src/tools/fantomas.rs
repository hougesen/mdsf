///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_fantomas_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("fantomas")];

    crate::execution::run_tools(&commands, file_path, timeout, set_fantomas_args)
}

#[cfg(test)]
mod test_fantomas {
    #[test_with::executable(fantomas)]
    fn test_fantomas_fsharp_ab29154716f5fe8a() {
        let input = r#"
let add a b  =  a +  b
            "#;
        let output = Some(
            r#"let add a b = a + b
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("fsharp");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::fantomas::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
