use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_golines(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("golines");

    cmd.arg("-w").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_golines {
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(golines)]
    #[test]
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
            setup_snippet(input, Language::Go.to_file_ext()).expect("it to create a snippet file");

        let output = super::format_using_golines(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
