use super::execute_command;
use crate::{error::MdsfError, runners::run_executable_from_path};

#[inline]
fn set_php_cs_fixer_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fix").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_php_cs_fixer(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_php_cs_fixer_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_php_cs_fixer(
        run_executable_from_path("tools/php-cs-fixer/vendor/bin/php-cs-fixer"),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) = invoke_php_cs_fixer(
        run_executable_from_path("vendor/bin/php-cs-fixer"),
        snippet_path,
    ) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_php_cs_fixer(std::process::Command::new("php-cs-fixer"), snippet_path)
}
