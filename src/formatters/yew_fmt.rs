use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("yew-fmt").build();

    // Needed for async
    cmd.arg("--edition").arg("2021");

    cmd.arg("--quiet").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_yew_fmt {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(yew-fmt)]
    fn it_should_format_rust() {
        let input = "pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    ";

        let expected_output = "pub async fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";

        let snippet =
            setup_snippet(input, language_to_ext("rust")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
