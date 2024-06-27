use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("ktlint");

    cmd.arg("--format")
        .arg("--log-level=error")
        .arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_ktlint {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(ktlint)]
    fn it_should_format_kotlin() {
        let input = "            fun add(a:Int ,b:Int ):Int {
                    return a + b
                }
            ";

        let expected_output = "

fun add(
    a: Int,
    b: Int,
): Int {
    return a + b
}
";

        let snippet =
            setup_snippet(input, language_to_ext("kotlin")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
