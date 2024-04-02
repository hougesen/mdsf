use crate::terminal::print_debug_formatter_info;

use super::execute_command;

#[inline]
pub fn format_using_fantomas(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_debug_formatter_info("fantomas");

    let mut cmd = std::process::Command::new("fantomas");

    cmd.arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_fantomas {
    use crate::{
        formatters::{fantomas::format_using_fantomas, setup_snippet},
        languages::Language,
    };

    #[test_with::executable(fantomas)]
    #[test]
    fn it_should_format_ocaml() {
        let input = "
let add a b  =  a +  b
            ";
        let expected_output = "let add a b = a + b
";

        let snippet = setup_snippet(input, Language::FSharp.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_fantomas(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
