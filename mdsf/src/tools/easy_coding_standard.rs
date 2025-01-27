///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_arguments(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("check");
    cmd.arg(file_path);
    cmd.arg("--fix");
    cmd.arg("--no-interaction");
    cmd
}

pub const COMMANDS: [CommandType; 2] = [CommandType::PhpVendor("ecs"), CommandType::Direct("ecs")];

#[cfg(test)]
mod test_easy_coding_standard {}
