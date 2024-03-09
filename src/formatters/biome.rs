use super::execute_command;

#[inline]
pub fn format_using_biome(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    // TODO: use installed biome instead
    let mut cmd = std::process::Command::new("npx");

    // Incase the user hasn't installed biome
    cmd.arg("--yes")
        .arg("@biomejs/biome")
        .arg("format")
        .arg("--write")
        .arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_biome {
    use crate::{
        formatters::{biome::format_using_biome, setup_snippet},
        languages::Language,
    };

    #[test]
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
\t// comments are allowed
\t\"key\": \"value\",
\t\"key2\": [\"value2\", \"value3\", 1, null]
}
";

        let snippet = setup_snippet(input, Language::Json.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_biome(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test]
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

        let snippet = setup_snippet(input, Language::JavaScript.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_biome(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test]
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

        let snippet = setup_snippet(input, Language::TypeScript.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_biome(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
