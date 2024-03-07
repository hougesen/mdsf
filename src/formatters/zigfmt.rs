use super::{execute_command, read_snippet};

pub fn format_using_zigfmt(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    let mut cmd = std::process::Command::new("zig");

    cmd.arg("fmt");

    cmd.arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}
