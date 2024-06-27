use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("gleam");

    cmd.arg("format").arg(snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[cfg(test)]
mod test_gleam_format {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(gleam)]
    async fn it_should_format_gleam() {
        let input = "pub fn add(a:Int,b:Int)->Int{a+b}";
        let expected_output = "pub fn add(a: Int, b: Int) -> Int {
  a + b
}
";

        let snippet = setup_snippet(input, language_to_ext("gleam"))
            .await
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
