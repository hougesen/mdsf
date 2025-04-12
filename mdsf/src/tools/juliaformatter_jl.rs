///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-E");
    let fps = file_path.to_string_lossy();
    cmd.arg(format!("using JuliaFormatter;format_file(\"{fps}\")"));
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("julia")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_juliaformatter_jl {
    #[test_with::executable(julia)]
    fn test_juliaformatter_jl_julia_6775294e3dc9244() {
        let input = r#"function add( a:: Int32,  b::Int32 )
            c = a+ b
            return c
            end "#;

        let output = r#"function add(a::Int32, b::Int32)
    c = a + b
    return c
end
"#;

        let file_ext = crate::fttype::get_file_extension("julia");

        crate::tools::Tooling::JuliaformatterJl.test_format_snippet(input, output, &file_ext);
    }
}
