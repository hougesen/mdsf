use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_yapf(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("yapf");

    cmd.arg("--in-place").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_yapf {
    use super::format_using_yapf;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(yapf)]
    #[test]
    fn it_should_format_python() {
        let input = "def add( a: int ,  b:int)->int: return a+b";

        let expected_output = "def add(a: int, b: int) -> int:\n    return a + b\n";

        let snippet = setup_snippet(input, Language::Python.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_yapf(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
