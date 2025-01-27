///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
fn set_beautysh_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

const COMMANDS: [CommandType; 1] = [CommandType::Direct("beautysh")];

#[inline]
pub fn run(
    file_path: &std::path::Path,
    timeout: u64,
) -> Result<(bool, Option<String>), crate::error::MdsfError> {
    crate::execution::run_tools(&COMMANDS, file_path, timeout, set_beautysh_args)
}

#[cfg(test)]
mod test_beautysh {
    #[test_with::executable(beautysh)]
    fn test_beautysh_shell_8e9f54ab33ca6912() {
        let input = r#"#!/bin/shell

       add() {
    echo "$1" + "$2"
             }
"#;
        let output = Some(
            r#"#!/bin/shell

add() {
    echo "$1" + "$2"
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("shell");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::beautysh::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(beautysh)]
    fn test_beautysh_bash_6cff8bc2ed5fa12f() {
        let input = r#"#!/bin/bash

       add() {
    echo "$1" + "$2"
             }
"#;
        let output = Some(
            r#"#!/bin/bash

add() {
    echo "$1" + "$2"
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("bash");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::beautysh::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
