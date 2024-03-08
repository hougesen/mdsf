use super::{execute_command, read_snippet};

#[inline]
pub fn format_using_prettier(snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
    // TODO: use installed prettier instead
    let mut cmd = std::process::Command::new("npx");

    // Incase the use hasn't installed prettier
    cmd.arg("--yes")
        .arg("prettier")
        .arg("--embedded-language-formatting")
        .arg("off")
        .arg("--write")
        .arg(snippet_path);

    if execute_command(&mut cmd)? {
        return read_snippet(snippet_path).map(Some);
    }

    Ok(None)
}

#[cfg(test)]
mod test_prettier {
    use crate::{
        formatters::{prettier::format_using_prettier, setup_snippet},
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
  // comments are allowed
  \"key\": \"value\",
  \"key2\": [\"value2\", \"value3\", 1, null],
}
";

        let snippet = setup_snippet(input, Language::Json.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path())
            .expect("it to be succesful")
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
  return a + b;
}
";

        let snippet = setup_snippet(input, Language::JavaScript.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path())
            .expect("it to be succesful")
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
  return a + b;
}
";

        let snippet = setup_snippet(input, Language::TypeScript.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path())
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test]
    fn it_should_format_markdown() {
        let input = "


# this is a title

this is a paragraph

            ";

        let expected_output = "# this is a title

this is a paragraph
";

        let snippet = setup_snippet(input, Language::Markdown.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path())
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test]
    fn it_should_not_format_code_snippets_in_markdown() {
        let input = "```typescript
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
    {
        return a+b
    }



```
";

        let snippet = setup_snippet(input, Language::Markdown.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path())
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(input, output);
    }

    #[test]
    fn it_should_format_html() {
        let input = " <!doctype html> <html> <head> <style> body {background-color: powderblue;} h1   {color: blue;} p    {color: red;} </style> </head> <body>  <h1>This is a heading</h1> <p>This is a paragraph.</p>  </body> </html> ";

        let expected_output = "<!doctype html>
<html>
  <head>
    <style>
      body {background-color: powderblue;} h1   {color: blue;} p    {color: red;}
    </style>
  </head>
  <body>
    <h1>This is a heading</h1>
    <p>This is a paragraph.</p>
  </body>
</html>
";

        let snippet = setup_snippet(input, Language::Html.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path())
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
