use mdsf::{config::MdsfConfig, error::MdsfError};

#[inline]
pub fn run() -> Result<(), MdsfError> {
    let config_path = std::env::current_dir()?.join("mdsf.json");

    if config_path.try_exists()? {
        return Err(MdsfError::ConfigAlreadyExist);
    }

    let config =
        serde_json::to_string_pretty(&MdsfConfig::default()).map_err(MdsfError::SerializeConfig)?;

    std::fs::write(config_path, config)?;

    Ok(())
}
