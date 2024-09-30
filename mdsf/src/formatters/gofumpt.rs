use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("gofumpt").build();

    cmd.arg("-w").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_gofumpt {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(gofumpt)]
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
            setup_snippet(input, &get_file_extension("go")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
