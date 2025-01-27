///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_fourmolu_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("fourmolu")];

    crate::execution::run_tools(&commands, file_path, timeout, set_fourmolu_args)
}

#[cfg(test)]
mod test_fourmolu {
    #[test_with::executable(fourmolu)]
    fn test_fourmolu_haskell_53dde041426fce49() {
        let input = r#"
addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;
        let output = Some(
            r#"addNumbers :: Int -> Int -> Int
addNumbers a b = do
    a + b
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("haskell");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::fourmolu::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
