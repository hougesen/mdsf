use super::execute_command;

#[inline]
pub fn format_using_stylish_haskell(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    let mut cmd = std::process::Command::new("stylish-haskell");

    cmd.arg("--inplace").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_stylish_haskell {
    use super::format_using_stylish_haskell;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(stylish-haskell)]
    #[test]
    fn it_should_format_haskell() {
        let input = "addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        ";

        let expected_output = "addNumbers::Int->Int->Int
addNumbers a b = do
        a + b

";

        let snippet = setup_snippet(input, Language::Haskell.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_stylish_haskell(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
