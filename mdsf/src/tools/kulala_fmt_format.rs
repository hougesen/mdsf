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

pub const COMMANDS: [CommandType; 6] = [
    CommandType::NodeModules("kulala-fmt"),
    CommandType::Direct("kulala-fmt"),
    CommandType::Npm("@mistweaverco/kulala-fmt"),
    CommandType::Pnpm("@mistweaverco/kulala-fmt"),
    CommandType::Bun("@mistweaverco/kulala-fmt"),
    CommandType::Deno("@mistweaverco/kulala-fmt"),
];

pub const IS_STDIN: bool = false;
