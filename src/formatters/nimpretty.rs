use super::execute_command;
use crate::terminal::print_formatter_info;

#[inline]
pub fn format_using_nimpretty(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("nimpretty");

    let mut cmd = std::process::Command::new("nimpretty");

    cmd.arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_nimpretty {
    use crate::{
        formatters::{nimpretty::format_using_nimpretty, setup_snippet},
        languages::Language,
    };

    #[test_with::executable(nimpretty)]
    #[test]
    fn it_should_format_nim() {
        let input = "proc           add( a         :int , b:int )        : int =
  return a +          b  ";

        let expected_output = "proc add(a: int, b: int): int =
  return a + b
";

        let snippet =
            setup_snippet(input, Language::Nim.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_nimpretty(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
