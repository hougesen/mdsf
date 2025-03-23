///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd.arg("--fix");
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("vsg"),
    CommandType::Uv("vsg", "vsg"),
    CommandType::Pipx("vsg"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_vhdl_style_guide {
    #[test_with::executable(vsg || pipx || uv)]
    fn test_vhdl_style_guide_vhd_7fa09a07176ec6() {
        let input = r#"
architecture RTL of FIFO is

  constant c_width : integer := 16;
  constant c_depth :   integer := 512;
  constant c_word :integer := 1024;

begin

end architecture RTL;"#;

        let output = r#"
architecture rtl of fifo is

  constant c_width : integer := 16;
  constant c_depth : integer := 512;
  constant c_word  : integer := 1024;

begin

end architecture rtl;
"#;

        let file_ext = crate::fttype::get_file_extension(".vhd");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::VhdlStyleGuide
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}
