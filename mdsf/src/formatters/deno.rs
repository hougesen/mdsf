use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run_fmt(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("deno").build();

    cmd.arg("fmt").arg("--quiet").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[inline]
pub fn run_lint(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("deno").build();

    cmd.arg("lint").arg("--fix").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_deno_fmt {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

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
            setup_snippet(input, &get_file_extension("json")).expect("it to create a snippet file");

        let output = super::run_fmt(snippet.path())
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

        let snippet = setup_snippet(input, &get_file_extension("javascript"))
            .expect("it to create a snippet file");

        let output = super::run_fmt(snippet.path())
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

        let snippet = setup_snippet(input, &get_file_extension("typescript"))
            .expect("it to create a snippet file");

        let output = super::run_fmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
