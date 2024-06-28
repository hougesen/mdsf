use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("crystal").build();

    cmd.arg("tool").arg("format").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_crystal_format {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(crystal)]
    fn it_should_format_crystal() {
        let input = "def add(a, b)  return a + b end";

        let expected_output = "def add(a, b)
  return a + b
end
";

        let snippet =
            setup_snippet(input, language_to_ext("crystal")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
