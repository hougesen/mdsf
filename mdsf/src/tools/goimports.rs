///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_goimports_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-w");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("goimports")];

    crate::execution::run_tools(&commands, file_path, timeout, set_goimports_args)
}

#[cfg(test)]
mod test_goimports {
    #[test_with::executable(goimports)]
    fn test_goimports_go_581afd5c8f294d82() {
        let input = r#"package main

import (
	"os"
	"fmt"
)

func add(a int, b int) int {
	fmt.Print(a)
	fmt.Print(b)
	return a + b
}
"#;
        let output = Some(
            r#"package main

import (
	"fmt"
)

func add(a int, b int) int {
	fmt.Print(a)
	fmt.Print(b)
	return a + b
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("go");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::goimports::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
