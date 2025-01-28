pub fn generate(path: &std::path::Path) -> anyhow::Result<String> {
    let output = std::process::Command::new("npx")
        .arg("--yes")
        .arg("markdown-toc")
        .arg(path)
        .output()?;

    let toc = String::from_utf8(output.stdout)?;

    Ok(toc)
}
