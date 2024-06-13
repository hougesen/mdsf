use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_deno_fmt(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("deno");

    cmd.arg("fmt").arg("--quiet").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_deno_fmt {
    use super::format_using_deno_fmt;
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(deno)]
    fn it_should_format_json() {
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

        let snippet =
            setup_snippet(input, &language_to_ext("json")).expect("it to create a snippet file");

        let output = format_using_deno_fmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(deno)]
    fn it_should_format_javascript() {
        let input = "
    async function asyncAddition(a,b){
        return a+b
    }

            ";

        let expected_output = "async function asyncAddition(a, b) {
  return a + b;
}
";

        let snippet = setup_snippet(input, &language_to_ext("javascript"))
            .expect("it to create a snippet file");

        let output = format_using_deno_fmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(deno)]
    fn it_should_format_typescript() {
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

        let snippet = setup_snippet(input, &language_to_ext("typescript"))
            .expect("it to create a snippet file");

        let output = format_using_deno_fmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
