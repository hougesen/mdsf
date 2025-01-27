///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_stylish_haskell_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--inplace");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("stylish-haskell")];

    crate::execution::run_tools(&commands, file_path, timeout, set_stylish_haskell_args)
}

#[cfg(test)]
mod test_stylish_haskell {
    #[test_with::executable(stylish-haskell)]
    fn test_stylish_haskell_haskell_72c790bb97bd7a26() {
        let input = r#"addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;
        let output = Some(
            r#"addNumbers::Int->Int->Int
addNumbers a b = do
        a + b

"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("haskell");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::stylish_haskell::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
