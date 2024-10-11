use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run_fmt(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("superhtml").build();

    cmd.arg("fmt").arg(file_path);

    execute_command(cmd, file_path)
}

#[cfg(test)]
mod test_superhtml {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(superhtml)]
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
</div>";

        let snippet =
            setup_snippet(input, &get_file_extension("html")).expect("it to create a snippet file");

        let output = super::run_fmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
