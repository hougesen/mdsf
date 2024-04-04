use super::execute_command;
use crate::terminal::print_formatter_info;

#[inline]
pub fn format_using_google_java_format(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("google-java-format");

    let mut cmd = std::process::Command::new("google-java-format");

    cmd.arg("-i").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_google_java_format {
    use super::format_using_google_java_format;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(google-java-format)]
    #[test]
    fn it_should_format_java() {
        let input = "class HelloWorld {
    public static void main(String[] args) {
                System.out.println(\"Hello\");
                System.out.println(\"World!\");
                 }
}";

        let expected_output = "class HelloWorld {
  public static void main(String[] args) {
    System.out.println(\"Hello\");
    System.out.println(\"World!\");
  }
}
";

        let snippet = setup_snippet(input, Language::Java.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_google_java_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
