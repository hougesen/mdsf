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
    cmd.arg(file_path);
    cmd.arg("--fix");
    cmd.arg("--no-interaction");
    cmd.arg("--quiet");
    cmd
}

pub const COMMANDS: [CommandType; 2] = [
    CommandType::PhpVendor("twig-cs-fixer"),
    CommandType::Direct("twig-cs-fixer"),
];
