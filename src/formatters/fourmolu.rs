use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("fourmolu").build();

    cmd.arg("-i").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_fourmolu {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(fourmolu)]
    fn it_should_format_haskell() {
        let input = "
addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        ";

        let expected_output = "addNumbers :: Int -> Int -> Int
addNumbers a b = do
    a + b
";

        let snippet =
            setup_snippet(input, language_to_ext("haskell")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
