use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("swiftformat");

    cmd.arg("--quiet").arg(snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[cfg(test)]
mod test_swiftformat {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(swiftformat)]
    async fn it_should_format_swift() {
        let input = " func add(a:Int ,b:Int)->Int {
    return a + b
    }";

        let expected_output = "func add(a: Int, b: Int) -> Int {
    return a + b
}
";

        let snippet = setup_snippet(input, language_to_ext("swift"))
            .await
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
