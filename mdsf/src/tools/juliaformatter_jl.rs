///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_juliaformatter_jl_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-E");
    let fps = file_path.to_string_lossy();
    cmd.arg(format!("using JuliaFormatter;format_file(\"{fps}\")"));
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("julia")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_juliaformatter_jl_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path);

        if index == commands.len() - 1 {
            return execution_result;
        }

        if let Ok(r) = execution_result {
            if !r.0 {
                return Ok(r);
            }
        }
    }

    Ok((true, None))
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
        let result = crate::tools::juliaformatter_jl::run(snippet.path())
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}
