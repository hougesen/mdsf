use super::{execute_command, read_snippet};

pub fn format_using_prettier(snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
    // TODO: use installed prettier instead
    let mut cmd = std::process::Command::new("npx");

    // Incase the use hasn't installed prettier
    cmd.arg("--yes")
        .arg("prettier")
        .arg("--embedded-language-formatting")
        .arg("off")
        .arg("--write")
        .arg(snippet_path);

    if execute_command(&mut cmd)? {
        return read_snippet(snippet_path).map(Some);
    }

    Ok(None)
}

