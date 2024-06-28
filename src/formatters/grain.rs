use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run_format(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("grain").build();

    cmd.arg("format").arg(file_path).arg("-o").arg(file_path);

    execute_command(cmd, file_path)
}

#[cfg(test)]
mod test_grain {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(grain)]
    fn it_should_format_grain() {
        let input = "module Hello

                                print(\"Hello, world!\")
";

        let expected_output = "module Hello

print(\"Hello, world!\")
";

        let snippet =
            setup_snippet(input, language_to_ext("grain")).expect("it to create a snippet file");

        let output = super::run_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
