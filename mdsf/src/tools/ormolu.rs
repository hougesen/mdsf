///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_arguments(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--mode");
    cmd.arg("inplace");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("ormolu")];

#[cfg(test)]
mod test_ormolu {
    #[test_with::executable(ormolu)]
    fn test_ormolu_haskell_c34a44cf19c5fdd7() {
        let input = r#"
addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;

        let output = r#"addNumbers :: Int -> Int -> Int
addNumbers a b = do
  a + b
"#;

        let file_ext = crate::fttype::get_file_extension("haskell");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result =
            crate::execution::run_tools(&super::COMMANDS, snippet.path(), super::set_arguments, 0)
                .expect("it to be successful")
                .1
                .expect("it to be some");

        assert_eq!(result, output);
    }
}
