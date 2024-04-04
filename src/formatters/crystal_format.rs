use super::execute_command;
use crate::terminal::print_formatter_info;

#[inline]
pub fn format_using_crystal_format(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("crystal");

    let mut cmd = std::process::Command::new("crystal");

    cmd.arg("tool").arg("format").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_crystal_format {
    use super::format_using_crystal_format;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(crystal)]
    #[test]
    fn it_should_format_crystal() {
        let input = "def add(a, b)  return a + b end";

        let expected_output = "def add(a, b)
  return a + b
end
";

        let snippet = setup_snippet(input, Language::Crystal.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_crystal_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
