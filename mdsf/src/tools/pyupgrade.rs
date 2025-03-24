///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("pyupgrade"),
    CommandType::Uv("pyupgrade", "pyupgrade"),
    CommandType::Pipx("pyupgrade"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_pyupgrade {
    #[test_with::executable(pyupgrade || pipx || uv)]
    fn test_pyupgrade_python_efcc3b576317ef09() {
        let input = r#"set([x for x in y])"#;

        let output = r#"{x for x in y}"#;

        let file_ext = crate::fttype::get_file_extension("python");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Pyupgrade
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
