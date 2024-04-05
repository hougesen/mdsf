use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_ktfmt(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("ktfmt");

    cmd.arg("--format")
        .arg("--log-level=error")
        .arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_ktfmt {
    use super::format_using_ktfmt;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(ktfmt)]
    #[test]
    fn it_should_format_kotlin() {
        let input = "            fun add(a:Int ,b:Int ):Int {
                    return a + b
                }
            ";

        let expected_output = "fun add(a: Int, b: Int): Int {
    return a + b
}
";

        let snippet = setup_snippet(input, Language::Kotlin.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_ktfmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
