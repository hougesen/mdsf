use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("htmlbeautifier").build();

    cmd.arg(file_path);

    execute_command(cmd, file_path)
}

#[cfg(test)]
mod test_htmlbeautifier {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(htmlbeautifier)]
    fn it_should_format_html() {
        let input = "<div>
                    <p>
                    Mads was here
                    </p>
        </div>";

        let expected_output = "<div>
  <p>
    Mads was here
  </p>
</div>
";

        let snippet =
            setup_snippet(input, language_to_ext("html")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
