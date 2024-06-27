use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("just");

    cmd.arg("--fmt")
        // TODO: remove once it is stabilized
        .arg("--unstable")
        .arg("--justfile")
        .arg(snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[cfg(test)]
mod test_just_fmt {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(just)]
    async fn it_should_format_just() {
        let input = "build:
                cargo build
                cargo build --release
            ";

        let expected_output = "build:
    cargo build
    cargo build --release
";

        let snippet = setup_snippet(input, language_to_ext("just"))
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
