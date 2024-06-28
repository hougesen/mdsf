use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_twig_cs_fixer_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("lint")
        .arg(snippet_path)
        .arg("--fix")
        .arg("--no-interaction")
        .arg("--quiet");

    cmd
}

#[inline]
fn invoke_twig_cs_fixer(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_twig_cs_fixer_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_twig_cs_fixer(
        CommandType::PhpVendor("twig-cs-fixer").build(),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_twig_cs_fixer(CommandType::Direct("twig-cs-fixer").build(), snippet_path)
}
