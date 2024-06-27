use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("fourmolu");

    cmd.arg("-i").arg(snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[cfg(test)]
mod test_fourmolu {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(fourmolu)]
    async fn it_should_format_haskell() {
        let input = "
addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        ";

        let expected_output = "addNumbers :: Int -> Int -> Int
addNumbers a b = do
    a + b
";

        let snippet = setup_snippet(input, language_to_ext("haskell"))
            .await
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
