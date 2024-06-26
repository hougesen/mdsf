use clap::Parser;
use mdsf::{
    cli::{Cli, Commands, FormatCommandArguments, LogLevel},
    error::MdsfError,
    terminal::logging::setup_logger,
};

mod completions;
mod format;
mod init;
mod prune_cache;

#[inline]
pub fn execute_command() -> Result<(), MdsfError> {
    match Cli::parse().command {
        Commands::Format(args) => {
            setup_logger(args.log_level.unwrap_or(LogLevel::Debug));

            format::run(args, false)
        }

        Commands::Verify(args) => {
            setup_logger(args.log_level.unwrap_or(LogLevel::Error));

            format::run(FormatCommandArguments::from(args), true)
        }

        Commands::Init => init::run().map_err(MdsfError::from),
        Commands::Completions(args) => {
            completions::run(&args);

            Ok(())
        }
        Commands::CachePrune => {
            prune_cache::run();

            Ok(())
        }
    }
}
