///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_arguments(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fmt");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("veryl")];

#[cfg(test)]
mod test_veryl_fmt {
    #[test_with::executable(veryl)]
    fn test_veryl_fmt_veryl_529de9cf882c5a00() {
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

        let result =
            crate::execution::run_tools(&super::COMMANDS, snippet.path(), super::set_arguments, 0)
                .expect("it to be successful")
                .1
                .expect("it to be some");

        assert_eq!(result, output);
    }
}
