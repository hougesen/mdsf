///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("rubyfmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_rubyfmt {
    #[test_with::executable(rubyfmt)]
    fn test_rubyfmt_ruby_d2b8a6db3c8eee1c() {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let file_ext = crate::fttype::get_file_extension("ruby");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Rubyfmt
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
