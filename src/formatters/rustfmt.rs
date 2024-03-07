use super::{execute_command, read_snippet};

pub fn format_using_rustfmt(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    let mut cmd = std::process::Command::new("rustfmt");

    // Needed for async
    cmd.arg("--edition").arg("2021");

    cmd.arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}
