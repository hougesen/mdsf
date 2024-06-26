use clap::{Args, Parser, Subcommand};

const HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{about-with-newline}{author-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, propagate_version = true, help_template = HELP_TEMPLATE)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run formatters on input files.
    Format(FormatCommandArguments),

    /// Verify files are formatted.
    Verify(VerifyCommandArguments),

    /// Create a new mdsf config.
    Init,

    /// Generate shell completion.
    Completions(CompletionsCommandArguments),

    /// Remove old caches.
    CachePrune(CachePruneArguments),
}

#[derive(Args, Debug)]
pub struct FormatCommandArguments {
    /// Path to file or directory
    #[arg()]
    pub path: std::path::PathBuf,

    /// Path to config
    #[arg(long)]
    pub config: Option<std::path::PathBuf>,

    /// Log stdout and stderr of formatters
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    #[arg(long, value_enum)]
    pub log_level: Option<LogLevel>,

    /// Amount of threads to use. Defaults to 0 (auto).
    #[arg(long)]
    pub threads: Option<usize>,

    /// Only format changed codeblocks
    #[arg(long, default_value_t = false)]
    pub cache: bool,
}

#[derive(Args, Debug)]
pub struct VerifyCommandArguments {
    /// Path to file or directory
    #[arg()]
    pub path: std::path::PathBuf,

    /// Path to config
    #[arg(long)]
    pub config: Option<std::path::PathBuf>,

    /// Log stdout and stderr of formatters
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    #[arg(long, value_enum)]
    pub log_level: Option<LogLevel>,

    /// Amount of threads to use. Defaults to 0 (auto).
    #[arg(long)]
    pub threads: Option<usize>,
}

impl From<VerifyCommandArguments> for FormatCommandArguments {
    #[inline]
    fn from(value: VerifyCommandArguments) -> Self {
        Self {
            path: value.path,
            config: value.config,
            debug: value.debug,
            log_level: value.log_level,
            threads: value.threads,
            cache: false,
        }
    }
}

#[derive(clap::ValueEnum, Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

#[derive(Args, Debug)]
pub struct CompletionsCommandArguments {
    pub shell: clap_complete::Shell,
}

#[derive(Args, Debug)]
pub struct CachePruneArguments {
    /// Remove caches that aren't state (based on config).
    #[arg(long, default_value_t = false)]
    pub all: bool,
}
