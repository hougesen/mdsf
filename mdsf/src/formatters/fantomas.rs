use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("fantomas").build();

    cmd.arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_fantomas {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(fantomas)]
    fn it_should_format_ocaml() {
        let input = "
let add a b  =  a +  b
            ";
        let expected_output = "let add a b = a + b
";

        let snippet = setup_snippet(input, &get_file_extension("fsharp"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
