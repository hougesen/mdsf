use mdsf::{
    cli::{ConfigFileFormat, InitCommandArguments},
    config::{MdsfConfig, schema_url},
    error::MdsfError,
};

#[inline]
pub fn run(args: &InitCommandArguments) -> Result<(), MdsfError> {
    if !args.force {
        raise_if_config_file_already_exists()?;
    }

    let default_config = MdsfConfig {
        schema: schema_url(args.schema_version),
        ..Default::default()
    };

    let serialized_config = match args.format {
        ConfigFileFormat::Json | ConfigFileFormat::Json5 | ConfigFileFormat::JsonC => {
            serde_json::to_string_pretty(&default_config).map_err(MdsfError::SerializeConfigJson)
        }
        ConfigFileFormat::Toml => {
            toml::to_string(&default_config).map_err(MdsfError::SerializeConfigToml)
        }
        ConfigFileFormat::Yaml => {
            serde_yaml::to_string(&default_config).map_err(MdsfError::SerializeConfigYaml)
        }
    }?;

    let config_path = std::path::Path::new("mdsf").with_extension(args.format.extension());

    std::fs::write(config_path, serialized_config).map_err(MdsfError::Io)
}

#[inline]
fn raise_if_config_file_already_exists() -> Result<(), MdsfError> {
    for name in MdsfConfig::supported_file_name() {
        let path = std::path::Path::new(name);

        if path.try_exists()? {
            return Err(MdsfError::ConfigAlreadyExist);
        }
    }

    Ok(())
}
