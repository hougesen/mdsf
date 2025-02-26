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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("beautysh")];

#[cfg(test)]
mod test_beautysh {
    #[test_with::executable(beautysh)]
    fn test_beautysh_shell_f8c934ee37e2888() {
        let input = r#"#!/bin/shell

       add() {
    echo "$1" + "$2"
             }
"#;

        let output = r#"#!/bin/shell

add() {
    echo "$1" + "$2"
}
"#;

        let file_ext = crate::fttype::get_file_extension("shell");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            0,
            false,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }

    #[test_with::executable(beautysh)]
    fn test_beautysh_bash_a6831a7ad31bd0a6() {
        let input = r#"#!/bin/bash

       add() {
    echo "$1" + "$2"
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

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            0,
            false,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}
