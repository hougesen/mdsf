///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 2] =
    [CommandType::PhpVendor("mago"), CommandType::Direct("mago")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_mago_format {
    #[test_with::executable(mago)]
    fn test_mago_format_php_17cf4527911d3cc9() {
        let input = r#"<?php
echo "Hello World!";
?>"#;

        let output = r#"<?php

echo 'Hello World!';
"#;

        let file_ext = crate::fttype::get_file_extension("php");

        crate::tools::Tooling::MagoFormat.test_format_snippet(input, output, &file_ext);
    }
}
