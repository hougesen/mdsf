use clap::Parser;
use mdsf::{
    cli::{Cli, Commands, FormatCommandArguments},
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
            setup_logger(args.log_level.unwrap_or_default());

            format::run(args, false)
        }

        Commands::Verify(args) => {
            setup_logger(args.log_level.unwrap_or_default());

            format::run(FormatCommandArguments::from(args), true)
        }

        Commands::Init => init::run().map_err(MdsfError::from),
        Commands::Completions(args) => {
            completions::run(&args);

            Ok(())
        }
        Commands::CachePrune(args) => {
            prune_cache::run(&args);

            Ok(())
        }
    }
}
