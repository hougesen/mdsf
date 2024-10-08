use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_prettier_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    let embedded_language_formatting = snippet_path.extension().is_some_and(|ext| ext != "md");

    if !embedded_language_formatting {
        cmd.arg("--embedded-language-formatting").arg("off");
    }

    cmd.arg("--log-level")
        .arg("error")
        .arg("--write")
        .arg(snippet_path);

    cmd
}

#[inline]
fn invoke_prettier(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_prettier_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_prettier(CommandType::NodeModules("prettier").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) = invoke_prettier(CommandType::Direct("prettier").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_prettier(CommandType::Npm("prettier").build(), snippet_path)
}

#[cfg(test)]
mod test_prettier {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(npx)]
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

        let snippet =
            setup_snippet(input, &get_file_extension("json")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
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
  return a + b;
}
";

        let snippet = setup_snippet(input, &get_file_extension("javascript"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
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
  return a + b;
}
";

        let snippet = setup_snippet(input, &get_file_extension("typescript"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(npx)]
    fn it_should_format_markdown() {
        let input = "


# this is a title

this is a paragraph

            ";

        let expected_output = "# this is a title

this is a paragraph
";

        let snippet = setup_snippet(input, &get_file_extension("markdown"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(npx)]
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

        let snippet = setup_snippet(input, &get_file_extension("markdown"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(input, output);
    }

    #[test_with::executable(npx)]
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

        let snippet =
            setup_snippet(input, &get_file_extension("html")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(npx)]
    fn it_should_format_css() {
        let input = " h1   {color: blue;} p    {color: red;} ";

        let expected_output = "h1 {
  color: blue;
}
p {
  color: red;
}
";

        let snippet =
            setup_snippet(input, &get_file_extension("css")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(npx)]
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

        let snippet =
            setup_snippet(input, &get_file_extension("yaml")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(npx)]
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
            setup_snippet(input, &get_file_extension("vue")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(npx)]
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

        let snippet = setup_snippet(input, &get_file_extension("graphql"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
