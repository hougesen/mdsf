use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_veryl_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("fmt");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("veryl")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_veryl_fmt_args(cmd.build(), file_path);
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
mod test_veryl_fmt {
    #[test_with::executable(veryl)]
    fn test_veryl_fmt_veryl_cc7688e3f0f92902() {
        let input = r#"/// documentation comment by markdown format
/// * list item1
/// * list item2
pub module Delay #( // visibility control by `pub` keyword
              param WIDTH:                 u32 = 1, // trailing comma is allowed
) (
    i_clk : input clock       ,
              i_rst : input reset       ,
    i_data: input logic<WIDTH>,
     o_data: input logic<WIDTH>,
)            {
    // unused variable which is not started with `_` are warned
              var _unused_variable: logic;

    // clock and reset signals can be omitted
    // because Veryl can infer these signals
                      always_ff              {
        // abstraction syntax of reset polarity and synchronicity
        if_reset {
            o_data = '0;
        } else {
            o_data = i_data;
        }
    }
}
"#;
        let output = r#"/// documentation comment by markdown format
/// * list item1
/// * list item2
pub module Delay #( // visibility control by `pub` keyword
    param WIDTH: u32 = 1, // trailing comma is allowed
) (
    i_clk : input clock       ,
    i_rst : input reset       ,
    i_data: input logic<WIDTH>,
    o_data: input logic<WIDTH>,
) {
    // unused variable which is not started with `_` are warned
    var _unused_variable: logic;

    // clock and reset signals can be omitted
    // because Veryl can infer these signals
    always_ff {
        // abstraction syntax of reset polarity and synchronicity
        if_reset {
            o_data = '0;
        } else {
            o_data = i_data;
        }
    }
}
"#;
        let file_ext = crate::fttype::get_file_extension("veryl");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::veryl_fmt::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}
