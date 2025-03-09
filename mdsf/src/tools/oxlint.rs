///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("oxlint"),
    CommandType::Direct("oxlint"),
    CommandType::Npm("oxlint"),
    CommandType::Pnpm("oxlint"),
    CommandType::Bun("oxlint"),
    CommandType::Deno("oxlint"),
    CommandType::Yarn("oxlint"),
];

pub const IS_STDIN: bool = false;
