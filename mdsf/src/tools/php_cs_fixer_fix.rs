///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_arguments(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 2] = [
    CommandType::PhpVendor("php-cs-fixer"),
    CommandType::Direct("php-cs-fixer"),
];

#[cfg(test)]
mod test_php_cs_fixer_fix {}
