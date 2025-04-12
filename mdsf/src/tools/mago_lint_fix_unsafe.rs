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
    fn test_mago_lint_fix_unsafe_php_8b9097d14e83ef96() {
        let input = r#"<?php
echo 'Hello World!';
"#;

        let output = r#"<?php

declare(strict_types=1);

echo 'Hello World!';
"#;

        let file_ext = crate::fttype::get_file_extension("php");

        crate::tools::Tooling::MagoLintFixUnsafe.test_format_snippet(input, output, &file_ext);
    }
}
