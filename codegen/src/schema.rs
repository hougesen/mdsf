use std::str::FromStr;

use anyhow::{Ok, Result};
use mdsf::config::MdsfConfig;

use crate::{cargo::get_package_version, tools::Tool};

pub fn generate() -> Result<()> {
    println!("generate schema");

    let package_version = get_package_version()?;

    let p = std::path::PathBuf::from_str(&format!("./schemas/v{package_version}"))?;

    std::fs::create_dir_all(&p)?;

    {
        let schema = serde_json::to_string_pretty(&schemars::schema_for!(MdsfConfig))?;

        std::fs::write(p.join("mdsf.schema.json"), schema)?;
    };

    {
        let schema = serde_json::to_string_pretty(&schemars::schema_for!(Tool))?;

        std::fs::write(
            std::path::PathBuf::from("tools").join("tool.schema.json"),
            schema,
        )?;
    };

    Ok(())
}
