use super::{execute_command, read_snippet};

pub fn format_using_stylua(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    let mut cmd = std::process::Command::new("stylua");

    cmd.arg("--verify");
    cmd.arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}
