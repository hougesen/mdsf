use crate::terminal::print_debug_formatter_info;

use super::execute_command;

#[inline]
pub fn format_using_autopep8(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_debug_formatter_info("autopep8");

    let mut cmd = std::process::Command::new("autopep8");

    cmd.arg("--in-place").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_autopep8 {
    use crate::{formatters::setup_snippet, languages::Language};

    use super::format_using_autopep8;

    #[test_with::executable(autopep8)]
    #[test]
    fn it_should_format_python() {
        let input = "def add( a: int ,  b:int)->int: return a+b";

        let expected_output = "def add(a: int,  b: int) -> int: return a+b\n";

        let snippet = setup_snippet(input, Language::Python.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_autopep8(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
