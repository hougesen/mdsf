use crate::terminal::print_formatter_info;

use super::execute_command;

#[inline]
pub fn format_using_ormolu(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("ormolu");

    let mut cmd = std::process::Command::new("ormolu");

    cmd.arg("--mode").arg("inplace").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_ormolu {
    use crate::{formatters::setup_snippet, languages::Language};

    use super::format_using_ormolu;

    #[test_with::executable(ormolu)]
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

        let output = format_using_ormolu(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
