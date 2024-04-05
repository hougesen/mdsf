use clap::{builder::OsStr, Parser};
use mdsf::{
    cli::{Cli, Commands, FormatCommandArguments},
    config::MdsfConfig,
    error::MdsfError,
    handle_file,
    terminal::{logging::setup_logger, print_error},
};

const MDSF_IGNORE_FILE_NAME: &str = ".mdsfignore";

fn format_command(args: FormatCommandArguments) -> Result<(), MdsfError> {
    mdsf::DEBUG.swap(args.debug, core::sync::atomic::Ordering::Relaxed);

    let conf = MdsfConfig::load()?;

    mdsf::runners::set_javascript_runtime(conf.javascript_runtime);

    if args.path.is_file() {
        handle_file(&conf, &args.path)?;
    } else if args.path.is_dir() {
        let mut walk_builder = ignore::WalkBuilder::new(args.path);

        walk_builder
            .standard_filters(true)
            .parents(true)
            .hidden(true)
            .add_custom_ignore_filename(MDSF_IGNORE_FILE_NAME);

        let ignore_path = std::env::current_dir()?.join(MDSF_IGNORE_FILE_NAME);

        if ignore_path.is_file() {
            walk_builder.add_ignore(ignore_path);
        }

        for entry in walk_builder.build().flatten() {
            let file_path = entry.path();

            if file_path.extension() == Some(&OsStr::from("md")) {
                handle_file(&conf, file_path)?;
            }
        }
    } else {
        return Err(MdsfError::FileNotFound(args.path));
    }

    Ok(())
}

fn init_config_command() -> std::io::Result<()> {
    let current_dir = std::env::current_dir()?;

    let conf = MdsfConfig::default();

    let config = serde_json::to_string_pretty(&conf)?;

    std::fs::write(current_dir.join("mdsf.json"), config)?;

    Ok(())
}

fn generate_schema_command() -> std::io::Result<()> {
    let mut p = std::env::current_dir()?;

    let package_version = env!("CARGO_PKG_VERSION");

    p.push(format!("schemas/v{package_version}"));

    std::fs::create_dir_all(&p)?;

    let schema = serde_json::to_string_pretty(&schemars::schema_for!(MdsfConfig))?;

    std::fs::write(p.join("mdsf.schema.json"), schema)?;

    Ok(())
}

fn main() {
    let command_result = match Cli::parse().command {
        Commands::Format(args) => {
            setup_logger(args.log_level);

            format_command(args)
        }
        Commands::Init => init_config_command().map_err(MdsfError::from),
        Commands::Schema => generate_schema_command().map_err(MdsfError::from),
    };

    if let Err(error) = command_result {
        print_error(&error);
    }
}
