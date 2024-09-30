use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run_fmt(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("veryl").build();

    cmd.arg("fmt").arg(file_path);

    execute_command(cmd, file_path)
}

#[cfg(test)]
mod test_veryl {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(veryl)]
    fn it_should_format_veryl() {
        let input = "/// documentation comment by markdown format
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
";

        let expected_output = "/// documentation comment by markdown format
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
";

        let snippet = setup_snippet(input, &get_file_extension("veryl"))
            .expect("it to create a snippet file");

        let output = super::run_fmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
