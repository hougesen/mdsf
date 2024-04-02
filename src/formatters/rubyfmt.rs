use crate::terminal::print_debug_formatter_info;

use super::execute_command;

#[inline]
pub fn format_using_rubyfmt(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_debug_formatter_info("rubyfmt");

    let mut cmd = std::process::Command::new("rubyfmt");

    cmd.arg("-i").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_rubyfmt {
    use crate::{
        formatters::{rubyfmt::format_using_rubyfmt, setup_snippet},
        languages::Language,
    };

    #[test_with::executable(rubyfmt)]
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

        let output = format_using_rubyfmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
