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

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_beautysh {
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

        crate::tools::Tooling::Beautysh.test_format_snippet(input, output, &file_ext);
    }

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

        crate::tools::Tooling::Beautysh.test_format_snippet(input, output, &file_ext);
    }
}
