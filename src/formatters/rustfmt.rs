use super::{execute_command, read_snippet};

pub fn format_using_rustfmt(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    let mut cmd = std::process::Command::new("rustfmt");

    // Needed for async
    cmd.arg("--edition").arg("2021");

    cmd.arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}

#[cfg(test)]
mod test_rustfmt {
    use crate::{formatters::setup_snippet, languages::Language};

    use super::format_using_rustfmt;

    #[test]
    fn it_should_format_code() {
        let snippet = r"pub     
                    async 
            fn    add( a: i32, 
                            b:i32 )->                   i32 {a+b}
    ";

        let expected_output = "pub async fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";

        let snippet = setup_snippet(snippet, Language::Rust.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_rustfmt(snippet.path())
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
