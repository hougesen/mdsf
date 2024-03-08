use mdsf::config::MdsfConfig;
use schemars::schema_for;

fn main() -> std::io::Result<()> {
    let mut p = std::env::current_dir()?;

    let package_version = env!("CARGO_PKG_VERSION");

    p.push(format!("schemas/v{package_version}"));

    std::fs::create_dir_all(&p)?;

    let schema = serde_json::to_string_pretty(&schema_for!(MdsfConfig))?;

    std::fs::write(p.join("mdsf.schema.json"), schema)?;

    Ok(())
}
