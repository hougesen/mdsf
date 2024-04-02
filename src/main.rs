use clap::{builder::OsStr, Parser};
use mdsf::{
    cli::{Cli, Commands, FormatCommandArguments},
    config::MdsfConfig,
    error::MdsfError,
    handle_file,
    terminal::print_file_not_found,
};

fn format_command(args: FormatCommandArguments) -> Result<(), MdsfError> {
    mdsf::DEBUG.swap(args.debug, core::sync::atomic::Ordering::Relaxed);

    let conf = MdsfConfig::load();

    if args.path.is_file() {
        handle_file(&conf, &args.path)?;
    } else if args.path.is_dir() {
        for entry in ignore::WalkBuilder::new(args.path)
            .git_ignore(true)
            .require_git(false)
            .hidden(true)
            .build()
            .flatten()
        {
            let file_path = entry.path();

            if file_path.extension() == Some(&OsStr::from("md")) {
                handle_file(&conf, file_path)?;
            }
        }
    } else {
        print_file_not_found(&args.path);
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
        Commands::Format(args) => format_command(args),
        Commands::Init => init_config_command().map_err(MdsfError::from),
        Commands::Schema => generate_schema_command().map_err(MdsfError::from),
    };

    if let Err(error) = command_result {
        eprintln!("{error}");
    }
}
