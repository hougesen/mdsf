///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-r");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("unimport"),
    CommandType::Uv("unimport", "unimport"),
    CommandType::Pipx("unimport"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_unimport {
    #[test_with::executable(unimport || pipx || uv)]
    fn test_unimport_python_55139008890abff0() {
        let input = r#"from typing import Optional
"#;

        let output = r#""#;

        let file_ext = crate::fttype::get_file_extension("python");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Unimport
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
