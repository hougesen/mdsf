use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run_format(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("ruff").build();

    cmd.arg("format").arg("--quiet").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[inline]
pub fn run_check(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("ruff").build();

    cmd.arg("check")
        .arg("--fix")
        .arg("--quiet")
        .arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_ruff {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(ruff)]
    fn it_should_format_python() {
        let input = "def add( a: int ,  b:int)->int: return a+b";

        let expected_output = "def add(a: int, b: int) -> int:\n    return a + b\n";

        let snippet = setup_snippet(input, &get_file_extension("python"))
            .expect("it to create a snippet file");

        let output = super::run_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
