use mdsf::{cli::InitCommandArguments, config::MdsfConfig, error::MdsfError};

#[inline]
pub fn run(args: &InitCommandArguments) -> Result<(), MdsfError> {
    let config_path = std::path::Path::new("mdsf.json");

    if !args.force && config_path.try_exists()? {
        return Err(MdsfError::ConfigAlreadyExist);
    }

    let config =
        serde_json::to_string_pretty(&MdsfConfig::default()).map_err(MdsfError::SerializeConfig)?;

    std::fs::write(config_path, config).map_err(MdsfError::Io)
}
