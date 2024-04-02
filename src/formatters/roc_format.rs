use crate::terminal::print_debug_formatter_info;

use super::execute_command;

#[inline]
pub fn format_using_roc_format(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_debug_formatter_info("roc_format");

    let mut cmd = std::process::Command::new("roc");

    cmd.arg("format").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_roc_format {
    use crate::{formatters::setup_snippet, languages::Language};

    use super::format_using_roc_format;

    #[test_with::executable(roc)]
    #[test]
    fn it_should_format_roc() {
        let input = r#"app "helloWorld"
    packages { pf: "https://github.com/roc-lang/" }
    imports [pf.Stdout]
    provides [main] to pf






main =
    Stdout.line "Hello, World!"


    "#;

        let expected_output = r#"app "helloWorld"
    packages { pf: "https://github.com/roc-lang/" }
    imports [pf.Stdout]
    provides [main] to pf

main =
    Stdout.line "Hello, World!"

"#;

        let snippet =
            setup_snippet(input, Language::Roc.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_roc_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
