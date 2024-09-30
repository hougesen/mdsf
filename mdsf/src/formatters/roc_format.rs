use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("roc").build();

    cmd.arg("format").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_roc_format {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(roc)]
    fn it_should_format_roc() {
        let input = r#"app "helloWorld"
    packages { pf: "https://github.com/roc-lang/" }
    imports [pf.Stdout]
    provides [main] to pf






main =
    Stdout.line "Hello, World!"


    "#;

        let expected_output = r#"app [main] { pf: platform "https://github.com/roc-lang/" }

import pf.Stdout

main =
    Stdout.line "Hello, World!"

"#;

        let snippet =
            setup_snippet(input, &get_file_extension("roc")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
