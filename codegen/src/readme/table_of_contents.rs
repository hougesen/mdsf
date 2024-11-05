pub fn generate() -> anyhow::Result<String> {
    let output = std::process::Command::new("npx")
        .arg("--yes")
        .arg("markdown-toc")
        .arg("README.md")
        .output()?;

    let toc = String::from_utf8(output.stdout)?;

    Ok(toc)
}
