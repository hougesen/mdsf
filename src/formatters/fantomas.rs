use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_fantomas(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("fantomas");

    cmd.arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_fantomas {
    use crate::{
        formatters::{fantomas::format_using_fantomas, setup_snippet},
        generated::language_to_ext,
    };

    #[test_with::executable(fantomas)]
    fn it_should_format_ocaml() {
        let input = "
let add a b  =  a +  b
            ";
        let expected_output = "let add a b = a + b
";

        let snippet =
            setup_snippet(input, &language_to_ext("fsharp")).expect("it to create a snippet file");

        let output = format_using_fantomas(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
