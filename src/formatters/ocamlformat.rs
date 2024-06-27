use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("ocamlformat");

    cmd.arg("--ignore-invalid-option")
        .arg("--inplace")
        .arg("--enable-outside-detected-project")
        .arg(snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[cfg(test)]
mod test_ocamlformat {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(ocamlformat)]
    async fn it_should_format_ocaml() {
        let input = "
let add a b  =  a +  b
            ";
        let expected_output = "let add a b = a + b
";

        let snippet = setup_snippet(input, language_to_ext("ocaml"))
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
