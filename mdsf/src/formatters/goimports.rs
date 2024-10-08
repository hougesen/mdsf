use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("goimports").build();

    cmd.arg("-w").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_goimports {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(goimports)]
    fn it_should_format_go() {
        let input = "package main

import (
\t\"os\"
\t\"fmt\"
)

func add(a int, b int) int {
\tfmt.Print(a)
\tfmt.Print(b)
\treturn a + b
}
";

        let expected_output = "package main

import (
\t\"fmt\"
)

func add(a int, b int) int {
\tfmt.Print(a)
\tfmt.Print(b)
\treturn a + b
}
";

        let snippet =
            setup_snippet(input, &get_file_extension("go")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
