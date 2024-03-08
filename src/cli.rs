use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Format(FormatCommandArguments),
    Init,
}

/// Run formatters on input files
#[derive(Args, Debug)]
pub struct FormatCommandArguments {
    #[arg()]
    pub path: std::path::PathBuf,
}
