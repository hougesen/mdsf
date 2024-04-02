use crate::terminal::print_debug_formatter_info;

use super::execute_command;

#[inline]
pub fn format_using_black(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_debug_formatter_info("black");

    let mut cmd = std::process::Command::new("black");

    cmd.arg("--quiet").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_black {
    use crate::{formatters::setup_snippet, languages::Language};

    use super::format_using_black;

    #[test_with::executable(black)]
    #[test]
    fn it_should_format_python() {
        let input = "def add( a: int ,  b:int)->int: return a+b";

        let expected_output = "def add(a: int, b: int) -> int:\n    return a + b\n";

        let snippet = setup_snippet(input, Language::Python.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_black(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
