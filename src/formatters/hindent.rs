use crate::terminal::print_formatter_info;

use super::execute_command;

#[inline]
pub fn format_using_hindent(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("hindent");

    let mut cmd = std::process::Command::new("hindent");

    cmd.arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_hindent {
    use crate::{formatters::setup_snippet, languages::Language};

    use super::format_using_hindent;

    #[test_with::executable(hindent)]
    #[test]
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

        let snippet = setup_snippet(input, Language::Haskell.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_hindent(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
