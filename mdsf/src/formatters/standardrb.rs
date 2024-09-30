use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("standardrb").build();

    cmd.arg("--fix").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_standardrb {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(standardrb)]
    fn it_should_format_ruby() {
        let input = "def   add(  a ,                                                          b )
                        return a + b
                end";

        let expected_output = "def add(a, b)
  a + b
end
";

        let snippet =
            setup_snippet(input, &get_file_extension("ruby")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
