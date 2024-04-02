use crate::{runners::setup_npm_script, terminal::print_formatter_info};

use super::execute_command;

#[inline]
pub fn format_using_standardjs(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("standardjs");

    let mut cmd = setup_npm_script("standard");

    cmd.arg("--fix").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_standardjs {
    use crate::{
        formatters::{setup_snippet, standardjs::format_using_standardjs},
        languages::{JavaScriptFlavor, Language},
    };

    #[test_with::executable(npx)]
    #[test]
    fn it_should_format_javascript() {
        let input = "
    async function asyncAddition(a,b  )
    {
        return a+b
    }

console.info(asyncAddition(1, 2));
            ";

        let expected_output = "async function asyncAddition (a, b) {
  return a + b
}

console.info(asyncAddition(1, 2))
";

        let snippet = setup_snippet(
            input,
            Language::JavaScript(JavaScriptFlavor::JavaScript).to_file_ext(),
        )
        .expect("it to create a snippet file");

        let output = format_using_standardjs(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
