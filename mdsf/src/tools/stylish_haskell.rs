///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--inplace");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("stylish-haskell")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_stylish_haskell {
    #[test_with::executable(stylish-haskell)]
    fn test_stylish_haskell_haskell_9589647c4239e2dd() {
        let input = r#"addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;

        let output = r#"addNumbers::Int->Int->Int
addNumbers a b = do
        a + b

"#;

        let file_ext = crate::fttype::get_file_extension("haskell");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::StylishHaskell
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}
