use super::execute_command;
use crate::terminal::print_formatter_info;

#[inline]
pub fn format_using_rufo(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("rufo");

    let mut cmd = std::process::Command::new("rufo");

    cmd.arg("--simple-exit").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_rufo {
    use crate::{
        formatters::{rufo::format_using_rufo, setup_snippet},
        languages::Language,
    };

    #[test_with::executable(rufo)]
    #[test]
    fn it_should_format_ruby() {
        let input = "def   add(  a ,                                                          b )
                        return a + b
                end";

        let expected_output = "def add(a, b)
  return a + b
end
";

        let snippet = setup_snippet(input, Language::Ruby.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_rufo(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
