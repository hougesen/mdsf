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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("shfmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_shfmt {
    #[test_with::executable(shfmt)]
    fn test_shfmt_shell_9c24a79abf093e10() {
        let input = r#"

#!/bin/sh

       add      ()   {
    echo "$1"                 +          "$2"
             }








"#;

        let output = r#"#!/bin/sh

add() {
	echo "$1" + "$2"
}
"#;

        let file_ext = crate::fttype::get_file_extension("shell");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Shfmt
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

    #[test_with::executable(shfmt)]
    fn test_shfmt_bash_9334f16dadf8ef68() {
        let input = r#"

#!/bin/bash

       add      ()   {
    echo "$1"                 +          "$2"
             }








"#;

        let output = r#"#!/bin/bash

add() {
	echo "$1" + "$2"
}
"#;

        let file_ext = crate::fttype::get_file_extension("bash");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Shfmt
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

    #[test_with::executable(shfmt)]
    fn test_shfmt_zsh_63d80ef78ac08ee0() {
        let input = r#"

#!/bin/zsh

       add      ()   {
    echo "$1"                 +          "$2"
             }








"#;

        let output = r#"#!/bin/zsh

add() {
	echo "$1" + "$2"
}
"#;

        let file_ext = crate::fttype::get_file_extension("zsh");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Shfmt
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
