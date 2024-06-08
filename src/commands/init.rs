use mdsf::config::MdsfConfig;

pub fn run() -> std::io::Result<()> {
    let current_dir = std::env::current_dir()?;

    let conf = MdsfConfig::default();

    let config = serde_json::to_string_pretty(&conf)?;

    std::fs::write(current_dir.join("mdsf.json"), config)?;

    Ok(())
}
