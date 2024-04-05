use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_ocp_indent(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("ocp-indent");

    cmd.arg("--inplace").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_ocp_indent {
    use crate::{
        formatters::{ocp_indent::format_using_ocp_indent, setup_snippet},
        languages::Language,
    };

    #[test_with::executable(ocp-indent)]
    #[test]
    fn it_should_format_ocaml() {
        let input = "
let add a b
                             = a + b
            ";
        let expected_output = "
let add a b
  = a + b
";

        let snippet = setup_snippet(input, Language::OCaml.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_ocp_indent(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
