use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("julia").build();

    cmd.arg("-E").arg(format!(
        "using JuliaFormatter;format_file(\"{}\")",
        snippet_path.to_string_lossy()
    ));

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_juliaformatter_jl {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(julia)]
    fn it_should_format_julia() {
        let input = "function add( a:: Int32,  b::Int32 )
            c = a+ b
            return c
            end ";

        let expected_output = "function add(a::Int32, b::Int32)
    c = a + b
    return c
end
";

        let snippet = setup_snippet(input, &get_file_extension("julia"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
