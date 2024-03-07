use super::{execute_command, read_snippet};

pub fn format_using_biome(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    // TODO: use installed biome instead
    let mut cmd = std::process::Command::new("npx");

    // Incase the use hasn't installed biome
    cmd.arg("--yes")
        .arg("@biomejs/biome")
        .arg("format")
        .arg("--write")
        .arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}
