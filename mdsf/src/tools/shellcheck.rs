///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("shellcheck")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_shellcheck {
    #[test_with::executable(shellcheck)]
    fn test_shellcheck_shell_7176996a1b8efe54() {
        let input = r#"#!/bin/sh

echo "Hello World"
"#;

        let output = r#"#!/bin/sh

echo "Hello World"
"#;

        let file_ext = crate::fttype::get_file_extension("shell");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Shellcheck
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
