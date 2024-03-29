use super::execute_command;

#[inline]
pub fn format_using_standardrb(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    let mut cmd = std::process::Command::new("standardrb");

    cmd.arg("--fix").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_standardrb {
    use crate::{
        formatters::{setup_snippet, standardrb::format_using_standardrb},
        languages::Language,
    };

    #[test_with::executable(standardrb)]
    #[test]
    fn it_should_format_ruby() {
        let input = "def   add(  a ,                                                          b )
                        return a + b
                end";

        let expected_output = "def add(a, b)
  a + b
end
";

        let snippet = setup_snippet(input, Language::Ruby.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_standardrb(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
