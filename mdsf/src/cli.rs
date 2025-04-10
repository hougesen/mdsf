use std::io::Read;

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

    #[arg(long, value_enum, global = true)]
    pub log_level: Option<LogLevel>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run formatters on input files.
    Format(FormatCommandArguments),

    /// Verify files are formatted.
    Verify(VerifyCommandArguments),

    /// Create a new mdsf config.
    Init(InitCommandArguments),

    /// Generate shell completion.
    Completions(CompletionsCommandArguments),

    /// Remove caches.
    CachePrune,
}

#[derive(Args, Debug)]
pub struct FormatCommandArguments {
    /// Path to files and/or directories.
    #[arg()]
    pub input: Vec<std::path::PathBuf>,

    /// Read input from stdin and write output to stdout
    #[arg(long, default_value_t = false)]
    pub stdin: bool,

    /// Path to config
    #[arg(long)]
    pub config: Option<std::path::PathBuf>,

    /// Log stdout and stderr of formatters
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Amount of threads to use.
    ///
    /// Defaults to 0 (auto).
    #[arg(long)]
    pub threads: Option<usize>,

    /// Cache results
    #[arg(long, default_value_t = false)]
    pub cache: bool,

    /// Tool timeout in seconds
    ///
    /// Defaults to no timeout
    #[arg(long)]
    pub timeout: Option<u64>,
}

#[derive(Args, Debug)]
pub struct VerifyCommandArguments {
    /// Path to files and/or directories.
    #[arg()]
    pub input: Vec<std::path::PathBuf>,

    /// Read input from stdin and write output to stdout
    #[arg(long, default_value_t = false)]
    pub stdin: bool,

    /// Path to config
    #[arg(long)]
    pub config: Option<std::path::PathBuf>,

    /// Log stdout and stderr of formatters
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Amount of threads to use.
    ///
    /// Defaults to 0 (auto).
    #[arg(long)]
    pub threads: Option<usize>,

    /// Tool timeout in seconds
    ///
    /// Defaults to no timeout
    #[arg(long)]
    pub timeout: Option<u64>,
}

impl From<VerifyCommandArguments> for FormatCommandArguments {
    #[inline]
    fn from(value: VerifyCommandArguments) -> Self {
        Self {
            cache: false,
            config: value.config,
            debug: value.debug,
            input: value.input,
            stdin: value.stdin,
            threads: value.threads,
            timeout: value.timeout,
        }
    }
}

#[derive(Args, Debug)]
pub struct InitCommandArguments {
    /// Create config even if one already exists in current directory
    #[arg(long, default_value_t = false)]
    pub force: bool,
}

#[derive(clap::ValueEnum, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LogLevel {
    Trace,
    #[default]
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Shell {
    /// Bourne Again `SHell` (bash)
    Bash,

    /// Elvish shell (elvish)
    Elvish,

    /// Friendly Interactive `SHell` (fish)
    Fish,

    /// `Nushell` (nushell)
    Nushell,

    /// `PowerShell` (powershell)
    PowerShell,

    /// Z `SHell` (zsh)
    Zsh,
}

impl clap::ValueEnum for Shell {
    #[inline]
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Bash,
            Self::Elvish,
            Self::Fish,
            Self::Nushell,
            Self::PowerShell,
            Self::Zsh,
        ]
    }

    #[inline]
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Bash => clap::builder::PossibleValue::new("bash"),
            Self::Elvish => clap::builder::PossibleValue::new("elvish"),
            Self::Fish => clap::builder::PossibleValue::new("fish"),
            Self::Nushell => clap::builder::PossibleValue::new("nushell"),
            Self::PowerShell => clap::builder::PossibleValue::new("powershell"),
            Self::Zsh => clap::builder::PossibleValue::new("zsh"),
        })
    }
}

#[derive(Args, Debug)]
pub struct CompletionsCommandArguments {
    pub shell: Shell,
}

#[derive(Args, Debug)]
pub struct CachePruneArguments {}

#[inline]
pub fn read_stdin() -> std::io::Result<String> {
    let stdin = std::io::stdin();

    let mut input = String::new();

    stdin.lock().read_to_string(&mut input)?;

    Ok(input)
}
