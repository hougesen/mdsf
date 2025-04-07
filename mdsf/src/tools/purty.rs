///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("purty"),
    CommandType::Direct("purty"),
    CommandType::Npm("purty"),
    CommandType::Pnpm("purty"),
    CommandType::Bun("purty"),
    CommandType::Deno("purty"),
    CommandType::Yarn("purty"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_purty {
    #[test_with::executable(purty || npx || pnpm || deno || bunx)]
    fn test_purty_purescript_37730dad0a7f9fbd() {
        let input = r#"module Mdsf where




add   :: Int -> Int  ->    Int
add a   b = a +         b
"#;

        let output = r#"module Mdsf where

add :: Int -> Int -> Int
add a b = a + b
"#;

        let file_ext = crate::fttype::get_file_extension("purescript");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Purty
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
