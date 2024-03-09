use super::execute_command;

#[inline]
pub fn format_using_rustfmt(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    let mut cmd = std::process::Command::new("rustfmt");

    // Needed for async
    cmd.arg("--edition").arg("2021");

    cmd.arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_rustfmt {
    use crate::{formatters::setup_snippet, languages::Language};

    use super::format_using_rustfmt;

    #[test]
    fn it_should_format_rust() {
        let input = "pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    ";

        let expected_output = "pub async fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";

        let snippet = setup_snippet(input, Language::Rust.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_rustfmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
