///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--force");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("smlfmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_smlfmt {
    #[test_with::executable(smlfmt)]
    fn test_smlfmt_sml_ca3c4a53d8aa2d76() {
        let input = r#"fun add(a:int, b: int )= a+b
"#;

        let output = r#"fun add (a: int, b: int) = a + b
"#;

        let file_ext = crate::fttype::get_file_extension(".sml");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Smlfmt
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
