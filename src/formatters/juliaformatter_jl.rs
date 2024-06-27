use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("julia");

    cmd.arg("-E").arg(format!(
        "using JuliaFormatter;format_file(\"{}\")",
        snippet_path.to_string_lossy()
    ));

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_juliaformatter_jl {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

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

        let snippet =
            setup_snippet(input, language_to_ext("julia")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
