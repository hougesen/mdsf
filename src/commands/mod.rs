use clap::Parser;
use mdsf::{
    cli::{Cli, Commands, LogLevel},
    error::MdsfError,
    terminal::logging::setup_logger,
};

mod check;
mod completions;
mod format;
mod init;
mod schema;

pub fn execute_command() -> Result<(), MdsfError> {
    match Cli::parse().command {
        Commands::Format(args) => {
            setup_logger(args.log_level.unwrap_or(LogLevel::Debug));

            format::run(args, false)
        }

        Commands::Verify(args) => {
            setup_logger(args.log_level.unwrap_or(LogLevel::Error));

            format::run(args, true)
        }

        Commands::Init => init::run().map_err(MdsfError::from),
        Commands::Schema => schema::run().map_err(MdsfError::from),
        Commands::Completions(args) => {
            completions::run(&args);

            Ok(())
        }
    }
}
