use super::execute_command;

#[inline]
pub fn format_using_ocamlformat(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    let mut cmd = std::process::Command::new("ocamlformat");

    cmd.arg("--ignore-invalid-option")
        .arg("--inplace")
        .arg("--quiet")
        .arg("--enable-outside-detected-project")
        .arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_ocamlformat {
    use crate::{
        formatters::{ocamlformat::format_using_ocamlformat, setup_snippet},
        languages::Language,
    };

    #[test]
    fn it_should_format_ocaml() {
        let input = "
let add a b  =  a +  b 
            ";
        let expected_output = "let add a b = a + b
";

        let snippet = setup_snippet(input, Language::OCaml.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_ocamlformat(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
