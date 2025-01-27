///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_shfmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("shfmt")];

    crate::execution::run_tools(&commands, file_path, timeout, set_shfmt_args)
}

#[cfg(test)]
mod test_shfmt {
    #[test_with::executable(shfmt)]
    fn test_shfmt_shell_a87ae97aff7e2d98() {
        let input = r#"

#!/bin/sh

       add      ()   {
    echo "$1"                 +          "$2"
             }








"#;
        let output = Some(
            r#"#!/bin/sh

add() {
	echo "$1" + "$2"
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("shell");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::shfmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(shfmt)]
    fn test_shfmt_bash_7888f1bdfe0dcaba() {
        let input = r#"

#!/bin/bash

       add      ()   {
    echo "$1"                 +          "$2"
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
        let result = crate::tools::shfmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(shfmt)]
    fn test_shfmt_zsh_a4d44f81b7f0ec51() {
        let input = r#"

#!/bin/zsh

       add      ()   {
    echo "$1"                 +          "$2"
             }








"#;
        let output = Some(
            r#"#!/bin/zsh

add() {
	echo "$1" + "$2"
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("zsh");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::shfmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
