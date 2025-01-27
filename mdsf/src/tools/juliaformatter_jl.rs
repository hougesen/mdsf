///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
fn set_juliaformatter_jl_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-E");
    let fps = file_path.to_string_lossy();
    cmd.arg(format!("using JuliaFormatter;format_file(\"{fps}\")"));
    cmd
}

const COMMANDS: [CommandType; 1] = [CommandType::Direct("julia")];

#[inline]
pub fn run(
    file_path: &std::path::Path,
    timeout: u64,
) -> Result<(bool, Option<String>), crate::error::MdsfError> {
    crate::execution::run_tools(&COMMANDS, file_path, timeout, set_juliaformatter_jl_args)
}

#[cfg(test)]
mod test_juliaformatter_jl {
    #[test_with::executable(julia)]
    fn test_juliaformatter_jl_julia_e931702b0e807c52() {
        let input = r#"function add( a:: Int32,  b::Int32 )
            c = a+ b
            return c
            end "#;
        let output = Some(
            r#"function add(a::Int32, b::Int32)
    c = a + b
    return c
end
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("julia");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::juliaformatter_jl::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
