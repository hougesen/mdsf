use super::{execute_command, read_snippet, setup_snippet};

pub fn format_using_nimpretty(code: &str) -> std::io::Result<Option<String>> {
    let file = setup_snippet(code)?;
    let file_path = file.path();

    let mut cmd = std::process::Command::new("nimpretty");

    cmd.arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}
