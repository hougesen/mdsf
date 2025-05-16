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
    cmd.arg("--kdl-version");
    cmd.arg("v2");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("kdlfmt"),
    CommandType::Direct("kdlfmt"),
    CommandType::Npm("kdlfmt"),
    CommandType::Pnpm("kdlfmt"),
    CommandType::Bun("kdlfmt"),
    CommandType::Deno("kdlfmt"),
    CommandType::Yarn("kdlfmt"),
];

pub const IS_STDIN: bool = false;
