use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_prettier_args(
    cmd: &mut std::process::Command,
    snippet_path: &std::path::Path,
    embedded_language_formatting: bool,
) {
    if !embedded_language_formatting {
        cmd.arg("--embedded-language-formatting").arg("off");
    }

    cmd.arg("--log-level")
        .arg("error")
        .arg("--write")
        .arg(snippet_path);
}

#[inline]
fn invoke_prettier(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
    embedded_language_formatting: bool,
) -> Result<(bool, Option<String>), MdsfError> {
    set_prettier_args(&mut cmd, snippet_path, embedded_language_formatting);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn format_using_prettier(
    snippet_path: &std::path::Path,
    embedded_language_formatting: bool,
) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_prettier(
        std::process::Command::new("prettier"),
        snippet_path,
        embedded_language_formatting,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_prettier(
        setup_npm_script("prettier"),
        snippet_path,
        embedded_language_formatting,
    )
}

#[cfg(test)]
mod test_prettier {
    use crate::{
        formatters::{prettier::format_using_prettier, setup_snippet},
        languages::{CssFlavor, JavaScriptFlavor, JsonFlavor, Language, TypeScriptFlavor},
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
  \"key2\": [\"value2\", \"value3\", 1, null]
}
";

        let snippet = setup_snippet(input, Language::Json(JsonFlavor::Json).to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path(), true)
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
  return a + b;
}
";

        let snippet = setup_snippet(
            input,
            Language::JavaScript(JavaScriptFlavor::JavaScript).to_file_ext(),
        )
        .expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path(), true)
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
  return a + b;
}
";

        let snippet = setup_snippet(
            input,
            Language::TypeScript(TypeScriptFlavor::TypeScript).to_file_ext(),
        )
        .expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path(), true)
            .expect("it to be successful")
            .1
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

        let output = format_using_prettier(snippet.path(), false)
            .expect("it to be successful")
            .1
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

        let output = format_using_prettier(snippet.path(), false)
            .expect("it to be successful")
            .1
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
      body {
        background-color: powderblue;
      }
      h1 {
        color: blue;
      }
      p {
        color: red;
      }
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

        let output = format_using_prettier(snippet.path(), true)
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test]
    fn it_should_format_css() {
        let input = " h1   {color: blue;} p    {color: red;} ";

        let expected_output = "h1 {
  color: blue;
}
p {
  color: red;
}
";

        let snippet = setup_snippet(input, Language::Css(CssFlavor::Css).to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path(), true)
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test]
    fn it_should_format_yaml() {
        let input = "


version:                                                                             2
updates:
  - package-ecosystem:                    \"cargo\"
    directory:  \"/\"
    schedule:
      interval:     \"monthly\"
    assignees:
      -     \"hougesen\"
    open-pull-requests-limit:       25

  - package-ecosystem:                              \"github-actions\"
    directory:          \"/\"
    schedule:
        interval:          \"monthly\"
    assignees:
        - \"hougesen\"
    open-pull-requests-limit: 25


        ";

        let expected_output = "version: 2
updates:
  - package-ecosystem: \"cargo\"
    directory: \"/\"
    schedule:
      interval: \"monthly\"
    assignees:
      - \"hougesen\"
    open-pull-requests-limit: 25

  - package-ecosystem: \"github-actions\"
    directory: \"/\"
    schedule:
      interval: \"monthly\"
    assignees:
      - \"hougesen\"
    open-pull-requests-limit: 25
";

        let snippet = setup_snippet(input, Language::Yaml.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path(), false)
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test]
    fn it_should_format_vue() {
        let input = "<script lang=\"ts\"   setup >
import {

    ref
} from \"vue\"


    const count   = ref(1)
    function add (a:number,b:number):number {
                return a +b
        }   </script>


<template>
    <button  @click=\"()=> count = add(count,count )\">Increment </button>
        </template>

";

        let expected_output = "<script lang=\"ts\" setup>
import { ref } from \"vue\";

const count = ref(1);
function add(a: number, b: number): number {
  return a + b;
}
</script>

<template>
  <button @click=\"() => (count = add(count, count))\">Increment</button>
</template>
";

        let snippet =
            setup_snippet(input, Language::Vue.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path(), true)
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test]
    fn it_should_format_graphql() {
        let input = "{   hero {     name
            # Queries can have comments!
         friends {       name     }   } }";

        let expected_output = "{
  hero {
    name
    # Queries can have comments!
    friends {
      name
    }
  }
}
";

        let snippet = setup_snippet(input, Language::GraphQL.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_prettier(snippet.path(), true)
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
