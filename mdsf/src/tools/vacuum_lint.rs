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
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("vacuum"),
    CommandType::Direct("vacuum"),
    CommandType::Npm("@quobix/vacuum"),
    CommandType::Pnpm("@quobix/vacuum"),
    CommandType::Bun("@quobix/vacuum"),
    CommandType::Deno("@quobix/vacuum"),
    CommandType::Yarn("@quobix/vacuum"),
];

pub const IS_STDIN: bool = false;
