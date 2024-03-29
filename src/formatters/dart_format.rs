use super::execute_command;

#[inline]
pub fn format_using_dart_format(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    let mut cmd = std::process::Command::new("dart");

    cmd.arg("format").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_dart_format {
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(dart)]
    #[test]
    fn it_should_format_dart() {
        let input = "class Adder {   int add(int a, int b) {     return a + b;   } }    ";

        let expected_output = "class Adder {
  int add(int a, int b) {
    return a + b;
  }
}
";

        let snippet = setup_snippet(input, Language::Dart.to_file_ext())
            .expect("it to create a snippet file");

        let output = super::format_using_dart_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
