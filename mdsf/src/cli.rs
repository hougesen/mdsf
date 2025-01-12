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
    /// Path to files and/or directories.
    #[arg()]
    pub input: Vec<std::path::PathBuf>,

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
    /// Path to files and/or directories.
    #[arg()]
    pub input: Vec<std::path::PathBuf>,

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
            input: value.input,
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
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Shell::Bash,
            Shell::Elvish,
            Shell::Fish,
            Shell::Nushell,
            Shell::PowerShell,
            Shell::Zsh,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Shell::Bash => clap::builder::PossibleValue::new("bash"),
            Shell::Elvish => clap::builder::PossibleValue::new("elvish"),
            Shell::Fish => clap::builder::PossibleValue::new("fish"),
            Shell::Nushell => clap::builder::PossibleValue::new("nushell"),
            Shell::PowerShell => clap::builder::PossibleValue::new("powershell"),
            Shell::Zsh => clap::builder::PossibleValue::new("zsh"),
        })
    }
}

#[derive(Args, Debug)]
pub struct CompletionsCommandArguments {
    pub shell: Shell,
}

#[derive(Args, Debug)]
pub struct CachePruneArguments {
    /// Remove caches that aren't state (based on config).
    #[arg(long, default_value_t = false)]
    pub all: bool,
}
