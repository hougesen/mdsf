use clap::{builder::OsStr, Parser};
use mdsf::{
    cli::{Cli, Commands, FormatCommandArguments},
    config::{init_config_command, MdsfConfig},
    error::MdsfError,
    format_file,
};

fn format_command(args: FormatCommandArguments) -> Result<(), MdsfError> {
    let conf = MdsfConfig::default();

    if args.path.is_file() {
        format_file(&conf, &args.path)?;
    } else {
        for entry in ignore::WalkBuilder::new(args.path)
            .git_ignore(true)
            .require_git(false)
            .hidden(true)
            .build()
            .flatten()
        {
            let file_path = entry.path();

            if file_path.extension() == Some(&OsStr::from("md")) {
                format_file(&conf, file_path)?;
            }
        }
    }

    Ok(())
}

fn main() {
    let command_result = match Cli::parse().command {
        Commands::Format(args) => format_command(args),
        Commands::Init => init_config_command().map_err(MdsfError::from),
    };

    if let Err(error) = command_result {
        eprintln!("{error}");
    }
}
