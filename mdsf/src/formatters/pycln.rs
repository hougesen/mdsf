use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("pycln").build();

    cmd.arg("--no-gitignore").arg("--quiet").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_pycln {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(pycln)]
    fn it_should_format_python() {
        let input = "import math";

        let expected_output = "";

        let snippet = setup_snippet(input, &get_file_extension("python"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
