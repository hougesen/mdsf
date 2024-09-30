use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("pyink").build();

    cmd.arg("--quiet").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_pyink {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(pyink)]
    fn it_should_format_python() {
        let input = "def add( a: int ,  b:int)->int: return a+b";

        let expected_output = "def add(a: int, b: int) -> int:\n    return a + b\n";

        let snippet = setup_snippet(input, &get_file_extension("python"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
