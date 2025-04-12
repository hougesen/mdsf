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

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize, Hash,
)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum OnMissingToolBinary {
    /// Allow missing binaries.
    #[default]
    #[serde(rename = "ignore")]
    Ignore,

    /// Exit with status code 1 when finished.
    #[serde(rename = "fail")]
    Fail,

    /// Instantly exit with status code 1.
    #[serde(rename = "fail-fast")]
    FailFast,
}

impl clap::ValueEnum for OnMissingToolBinary {
    #[inline]
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Ignore, Self::Fail, Self::FailFast]
    }

    #[inline]
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Ignore => clap::builder::PossibleValue::new("ignore"),
            Self::Fail => clap::builder::PossibleValue::new("fail"),
            Self::FailFast => clap::builder::PossibleValue::new("fail-fast"),
        })
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize, Hash,
)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum OnMissingLanguageDefinition {
    /// Allow missing binaries.
    #[default]
    #[serde(rename = "ignore")]
    Ignore,

    /// Exit with status code 1 when finished.
    #[serde(rename = "fail")]
    Fail,

    /// Instantly exit with status code 1.
    #[serde(rename = "fail-fast")]
    FailFast,
}

impl clap::ValueEnum for OnMissingLanguageDefinition {
    #[inline]
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Ignore, Self::Fail, Self::FailFast]
    }

    #[inline]
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Ignore => clap::builder::PossibleValue::new("ignore"),
            Self::Fail => clap::builder::PossibleValue::new("fail"),
            Self::FailFast => clap::builder::PossibleValue::new("fail-fast"),
        })
    }
}

#[derive(Args, Debug)]
pub struct FormatCommandArguments {
    /// Path to files and/or directories.
    #[arg()]
    pub input: Vec<std::path::PathBuf>,

    /// Read input from stdin and write output to stdout.
    #[arg(long, default_value_t = false)]
    pub stdin: bool,

    /// Path to config file.
    #[arg(long)]
    pub config: Option<std::path::PathBuf>,

    /// Log stdout and stderr of formatters.
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

    /// Tool timeout in seconds.
    ///
    /// Defaults to no timeout.
    #[arg(long)]
    pub timeout: Option<u64>,

    /// What to do when a codeblock language has no tools defined.
    #[arg(long)]
    pub on_missing_language_definition: Option<OnMissingLanguageDefinition>,

    /// What to do when the binary of a tool cannot be found.
    #[arg(long)]
    pub on_missing_tool_binary: Option<OnMissingToolBinary>,
}

#[derive(Args, Debug)]
pub struct VerifyCommandArguments {
    /// Path to files and/or directories.
    #[arg()]
    pub input: Vec<std::path::PathBuf>,

    /// Read input from stdin and write output to stdout.
    #[arg(long, default_value_t = false)]
    pub stdin: bool,

    /// Path to config file.
    #[arg(long)]
    pub config: Option<std::path::PathBuf>,

    /// Log stdout and stderr of formatters.
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Amount of threads to use.
    ///
    /// Defaults to 0 (auto).
    #[arg(long)]
    pub threads: Option<usize>,

    /// Tool timeout in seconds.
    ///
    /// Defaults to no timeout.
    #[arg(long)]
    pub timeout: Option<u64>,

    /// What to do when a codeblock language has no tools defined.
    #[arg(long)]
    pub on_missing_language_definition: Option<OnMissingLanguageDefinition>,

    /// What to do when the binary of a tool cannot be found.
    #[arg(long)]
    pub on_missing_tool_binary: Option<OnMissingToolBinary>,
}

impl From<VerifyCommandArguments> for FormatCommandArguments {
    #[inline]
    fn from(value: VerifyCommandArguments) -> Self {
        Self {
            cache: false,
            config: value.config,
            debug: value.debug,
            input: value.input,
            on_missing_language_definition: value.on_missing_language_definition,
            on_missing_tool_binary: value.on_missing_tool_binary,
            stdin: value.stdin,
            threads: value.threads,
            timeout: value.timeout,
        }
    }
}

#[derive(Args, Debug)]
pub struct InitCommandArguments {
    /// Create config even if one already exists in current directory.
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
    /// Bourne Again `SHell` (bash).
    Bash,

    /// Elvish shell (elvish).
    Elvish,

    /// Friendly Interactive `SHell` (fish).
    Fish,

    /// `Nushell` (nushell).
    Nushell,

    /// `PowerShell` (powershell).
    PowerShell,

    /// Z `SHell` (zsh).
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
