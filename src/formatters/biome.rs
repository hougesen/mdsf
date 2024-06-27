use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
pub fn run_format(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    // NOTE: the biome docs recommend running biome using npx, and not directly
    let mut cmd = setup_npm_script("@biomejs/biome");

    cmd.arg("format").arg("--write").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn run_lint(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    // NOTE: the biome docs recommend running biome using npx, and not directly
    let mut cmd = setup_npm_script("@biomejs/biome");

    cmd.arg("lint").arg("--write").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn run_check(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    // NOTE: the biome docs recommend running biome using npx, and not directly
    let mut cmd = setup_npm_script("@biomejs/biome");

    cmd.arg("check").arg("--write").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_biome {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(npx)]
    fn it_should_format_json() {
        let input = "
              {
              \"key\": \"value\",
  \"key2\": [
      \"value2\",
      \"value3\",
      1
            , null]
 }
  ";

        let expected_output = "{
\t\"key\": \"value\",
\t\"key2\": [\"value2\", \"value3\", 1, null]
}
";

        let snippet =
            setup_snippet(input, language_to_ext("json")).expect("it to create a snippet file");

        let output = super::run_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(npx)]
    fn it_should_format_javascript() {
        let input = "
    async function asyncAddition(
            a,b
        ) {
        return a+b
    }

            ";

        let expected_output = "async function asyncAddition(a, b) {
\treturn a + b;
}
";

        let snippet = setup_snippet(input, language_to_ext("javascript"))
            .expect("it to create a snippet file");

        let output = super::run_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(npx)]
    fn it_should_format_typescript() {
        let input = "
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
    {
        return a+b
    }

            ";

        let expected_output =
            "async function asyncAddition(a: number, b: number): Promise<number> {
\treturn a + b;
}
";

        let snippet = setup_snippet(input, language_to_ext("typescript"))
            .expect("it to create a snippet file");

        let output = super::run_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
