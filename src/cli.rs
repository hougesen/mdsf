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
    /// Create a new mdsf config
    Init,
    /// Generate json schema
    Schema,
}

/// Run formatters on input files
#[derive(Args, Debug)]
pub struct FormatCommandArguments {
    // Path to file or directory
    #[arg()]
    pub path: std::path::PathBuf,

    /// Log stdout and stderr of formatters
    #[arg(long, default_value_t = false)]
    pub debug: bool,
}
