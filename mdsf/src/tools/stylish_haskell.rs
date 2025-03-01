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

#[cfg(test)]
mod test_stylish_haskell {
    const TIMEOUT: u64 = 0;
    const DEBUG_ENABLED: bool = true;

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

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            false,
            DEBUG_ENABLED,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
