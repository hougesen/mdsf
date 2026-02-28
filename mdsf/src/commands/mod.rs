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
pub fn run_command() -> Result<(), MdsfError> {
    let c = Cli::parse();

    setup_logger(c.log_level);

    match c.command {
        Commands::CachePrune => prune_cache::run().map_err(MdsfError::from),
        Commands::Completions(args) => {
            completions::run(&args, &mut std::io::stdout());

            Ok(())
        }
        Commands::Format(args) => format::run(args, false),
        Commands::Init(args) => init::run(&args),
        Commands::Verify(args) => format::run(FormatCommandArguments::from(args), true),
    }
}
