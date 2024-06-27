use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run_fmt(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("deno");

    cmd.arg("fmt").arg("--quiet").arg(snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[inline]
pub async fn run_lint(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("deno");

    cmd.arg("lint").arg("--fix").arg(snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[cfg(test)]
mod test_deno_fmt {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(deno)]
    async fn it_should_format_json() {
        let input = "
              {
    // comments are allowed
              \"key\": \"value\",
  \"key2\": [
      \"value2\",
      \"value3\",
      1
            , null]
 }
  ";

        let expected_output = "{
  // comments are allowed
  \"key\": \"value\",
  \"key2\": [
    \"value2\",
    \"value3\",
    1,
    null
  ]
}
";

        let snippet = setup_snippet(input, language_to_ext("json"))
            .await
            .expect("it to create a snippet file");

        let output = super::run_fmt(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[tokio::test]
    #[test_with::executable(deno)]
    async fn it_should_format_javascript() {
        let input = "
    async function asyncAddition(a,b){
        return a+b
    }

            ";

        let expected_output = "async function asyncAddition(a, b) {
  return a + b;
}
";

        let snippet = setup_snippet(input, language_to_ext("javascript"))
            .await
            .expect("it to create a snippet file");

        let output = super::run_fmt(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[tokio::test]
    #[test_with::executable(deno)]
    async fn it_should_format_typescript() {
        let input = "
    async function asyncAddition(                                a:       \tnumber,b:number ) :Promise< number>
    {
        return a+b
    }

            ";

        let expected_output =
            "async function asyncAddition(a: number, b: number): Promise<number> {
  return a + b;
}
";

        let snippet = setup_snippet(input, language_to_ext("typescript"))
            .await
            .expect("it to create a snippet file");

        let output = super::run_fmt(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
