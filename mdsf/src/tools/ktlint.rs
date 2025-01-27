///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_ktlint_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--format");
    cmd.arg("--log-level=error");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("ktlint")];

    crate::execution::run_tools(&commands, file_path, timeout, set_ktlint_args)
}

#[cfg(test)]
mod test_ktlint {
    #[test_with::executable(ktlint)]
    fn test_ktlint_kotlin_6b60db8bc8c2a271() {
        let input = r#"            fun add(a:Int ,b:Int ):Int {
                    return a + b
                }
            "#;
        let output = Some(
            r#"

fun add(
    a: Int,
    b: Int,
): Int {
    return a + b
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("kotlin");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::ktlint::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
