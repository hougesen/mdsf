use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_gofmt(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("gofmt");

    cmd.arg("-w").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_gofmt {
    use crate::{
        formatters::{gofmt::format_using_gofmt, setup_snippet},
        generated::language_to_ext,
    };

    #[test_with::executable(gofmt)]
    #[test]
    fn it_should_format_go() {
        let input = "package main

   func add(a int , b int  ) int {
                return a + b
       }

    ";

        let expected_output = "package main

func add(a int, b int) int {
\treturn a + b
}
";

        let snippet =
            setup_snippet(input, &language_to_ext("go")).expect("it to create a snippet file");

        let output = format_using_gofmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
