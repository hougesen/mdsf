use mdsf::config::MdsfConfig;

use crate::{error::CodegenError, tools::Tool};

pub fn generate() -> Result<(), CodegenError> {
    println!("generate schema");

    let package_version = env!("CARGO_PKG_VERSION");

    let schema = serde_json::to_string_pretty(&schemars::schema_for!(MdsfConfig))?;

    {
        let version_dir = std::path::PathBuf::from(&format!("./schemas/v{package_version}"));

        std::fs::create_dir_all(&version_dir)?;

        std::fs::write(version_dir.join("mdsf.schema.json"), &schema)?;
    };

    {
        let dev_dir = std::path::PathBuf::from("./schemas/development");

        std::fs::create_dir_all(&dev_dir)?;

        std::fs::write(dev_dir.join("mdsf.schema.json"), schema)?;
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
