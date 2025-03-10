///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("lint");
    cmd.arg("--fix");
    cmd.arg("--potentially-unsafe");
    cmd.arg("--unsafe");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 2] =
    [CommandType::PhpVendor("mago"), CommandType::Direct("mago")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_mago_lint_fix_unsafe {
    #[test_with::executable(mago)]
    fn test_mago_lint_fix_unsafe_php_513b2cc3a1e145ed() {
        let input = r#"<?php
echo 'Hello World!';
"#;

        let output = r#"<?php
echo 'Hello World!';
"#;

        let file_ext = crate::fttype::get_file_extension("php");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::MagoLintFixUnsafe
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
