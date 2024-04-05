use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_swift_format(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("swift-format");

    cmd.arg("--in-place").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_swift_format {
    use super::format_using_swift_format;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(swift_format)]
    #[test]
    fn it_should_format_swift() {
        let input = " func add(a:Int ,b:Int)->Int {
    return a + b
    }";

        let expected_output = "func add(a: Int, b: Int) -> Int {
    return a + b
}
";

        let snippet = setup_snippet(input, Language::Swift.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_swift_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}