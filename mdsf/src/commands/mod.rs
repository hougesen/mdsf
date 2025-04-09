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
pub fn run_command() -> Result<(), MdsfError> {
    match Cli::parse().command {
        Commands::Format(args) => {
            setup_logger(args.log_level.unwrap_or_else(|| {
                if args.stdin {
                    LogLevel::Error
                } else {
                    LogLevel::default()
                }
            }));

            format::run(args, false)
        }

        Commands::Verify(args) => {
            setup_logger(args.log_level.unwrap_or_else(|| {
                if args.stdin {
                    LogLevel::Error
                } else {
                    LogLevel::default()
                }
            }));

            format::run(FormatCommandArguments::from(args), true)
        }

        Commands::Init(args) => {
            setup_logger(LogLevel::default());

            init::run(&args)
        }
        Commands::Completions(args) => {
            completions::run(&args, &mut std::io::stdout());

            Ok(())
        }
        Commands::CachePrune => prune_cache::run().map_err(MdsfError::from),
    }
}
