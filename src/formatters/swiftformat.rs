use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_swiftformat(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("swiftformat");

    cmd.arg("--quiet").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_swiftformat {
    use super::format_using_swiftformat;
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(swiftformat)]
    fn it_should_format_swift() {
        let input = " func add(a:Int ,b:Int)->Int {
    return a + b
    }";

        let expected_output = "func add(a: Int, b: Int) -> Int {
    return a + b
}
";

        let snippet =
            setup_snippet(input, &language_to_ext("swift")).expect("it to create a snippet file");

        let output = format_using_swiftformat(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
