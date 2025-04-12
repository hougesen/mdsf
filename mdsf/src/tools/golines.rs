///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-w");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("golines")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_golines {
    #[test_with::executable(golines)]
    fn test_golines_go_4af43f410d7fff15() {
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

        let output = r#"package main

import (
	"fmt"
)

func add(a int, b int) int {
	fmt.Print(a)
	fmt.Print(b)
	return a + b
}
"#;

        let file_ext = crate::fttype::get_file_extension("go");

        crate::tools::Tooling::Golines.test_format_snippet(input, output, &file_ext);
    }
}
