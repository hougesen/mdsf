use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("google-java-format");

    cmd.arg("-i").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_google_java_format {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(google-java-format)]
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

        let snippet =
            setup_snippet(input, language_to_ext("java")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
