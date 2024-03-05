use crate::languages::read_snippet;

use super::{execute_command, setup_snippet};

pub fn format_using_rustfmt(code: &str) -> std::io::Result<Option<String>> {
    let file = setup_snippet(code)?;
    let file_path = file.path();

    let mut cmd = std::process::Command::new("rustfmt");

    // Needed for async
    cmd.arg("--edition").arg("2021");

    cmd.arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}
