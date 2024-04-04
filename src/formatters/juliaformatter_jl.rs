use super::execute_command;
use crate::terminal::print_formatter_info;

#[inline]
pub fn format_using_juliaformatter_jl(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("juliaformatter.jl");

    let mut cmd = std::process::Command::new("julia");

    cmd.arg("-E").arg(format!(
        "using JuliaFormatter;format_file(\"{}\")",
        snippet_path.to_string_lossy()
    ));

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_juliaformatter_jl {
    use super::format_using_juliaformatter_jl;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(julia)]
    #[test]
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

        let snippet = setup_snippet(input, Language::Julia.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_juliaformatter_jl(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
