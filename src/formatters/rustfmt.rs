use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("rustfmt");

    // Needed for async
    cmd.arg("--edition").arg("2021");

    cmd.arg("--quiet").arg(snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[cfg(test)]
mod test_rustfmt {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(rustfmt)]
    async fn it_should_format_rust() {
        let input = "pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    ";

        let expected_output = "pub async fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";

        let snippet = setup_snippet(input, language_to_ext("rust"))
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
