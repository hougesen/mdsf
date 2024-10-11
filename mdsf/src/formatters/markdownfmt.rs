use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("markdownfmt").build();

    cmd.arg("-w").arg(file_path);

    execute_command(cmd, file_path)
}

#[cfg(test)]
mod test_markdownfmt {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(markdownfmt)]
    fn it_should_format_html() {
        let input = "# hello w   world

this   text has      weird spacing

- first
* second";

        let expected_output = "hello w world
=============

this text has weird spacing

-	first
-	second
";

        let snippet = setup_snippet(input, &get_file_extension("markdown"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
