use super::{execute_command, read_snippet};

pub fn format_using_ruff(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    let mut cmd = std::process::Command::new("ruff");

    cmd.arg("format");
    cmd.arg("--quiet");
    cmd.arg("--no-cache");
    cmd.arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}
