use std::process::Command;

use super::{execute_command, read_snippet, setup_snippet};

pub fn format_using_ruff(code: &str) -> std::io::Result<Option<String>> {
    let file = setup_snippet(code)?;
    let file_path = file.path();

    let mut cmd = Command::new("ruff");

    cmd.arg("format");
    cmd.arg("--quiet");
    cmd.arg("--no-cache");
    cmd.arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}
