use super::execute_command;

#[inline]
pub fn format_using_rubocop(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    let mut cmd = std::process::Command::new("rubocop");

    cmd.arg("--fix-layout")
        .arg("--autocorrect")
        .arg("--format")
        .arg("quiet")
        .arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_ruff {
    use crate::{
        formatters::{rubocop::format_using_rubocop, setup_snippet},
        languages::Language,
    };

    #[test]
    fn it_should_format_rust() {
        let input = "def   add(  a ,                                                          b )
                        return a + b
                end";

        let expected_output = "def add(a, b)
  return a + b
end
";

        let snippet = setup_snippet(input, Language::Ruby.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_rubocop(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
