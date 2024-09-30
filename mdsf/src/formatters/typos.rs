use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("typos").build();

    cmd.arg("-w")
        .arg("--no-ignore")
        .arg("--hidden")
        .arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_typos {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(typos)]
    fn it_should_fix_typos() {
        let input = "anouncement
";

        let expected_output = "announcement
";
        let snippet = setup_snippet(input, &get_file_extension("python"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
