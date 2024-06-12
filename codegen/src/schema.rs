use std::str::FromStr;

use anyhow::{Ok, Result};
use mdsf::config::MdsfConfig;

#[derive(serde::Deserialize)]
struct Package {
    version: String,
}

#[derive(serde::Deserialize)]
struct Cargo {
    package: Package,
}

fn get_package_version() -> Result<String> {
    let file = std::fs::read_to_string("../Cargo.toml")?;

    let config = toml::from_str::<Cargo>(&file)?;

    Ok(config.package.version)
}

pub fn generate() -> Result<()> {
    println!("generate schema");

    let package_version = get_package_version()?;

    let p = std::path::PathBuf::from_str(&format!("../schemas/v{package_version}"))?;

    std::fs::create_dir_all(&p)?;

    let schema = serde_json::to_string_pretty(&schemars::schema_for!(MdsfConfig))?;

    std::fs::write(p.join("mdsf.schema.json"), schema)?;

    Ok(())
}
