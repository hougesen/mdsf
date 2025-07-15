use crate::terminal::print_error;

pub static HAS_ERROR: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

#[inline]
pub fn set_exit_code_error() {
    HAS_ERROR.swap(true, core::sync::atomic::Ordering::Relaxed);
}

#[inline]
pub fn exit_with_error(error: &MdsfError) -> ! {
    print_error(error);

    std::process::exit(1)
}

#[derive(Debug)]
pub enum MdsfError {
    CheckModeChanges(u32),
    ConfigAlreadyExist,
    ConfigNotFound(std::path::PathBuf),
    // TODO: use &std::path::Path
    ConfigParse((std::path::PathBuf, serde_json::Error)),
    Io(std::io::Error),
    /// Another alias clashes
    LanguageAliasClash(String, String, String),
    // TODO: rename 😅
    /// `languages` contains the language
    LanguageAliasLanguagesContainsLanguage(String),
    LanguageAliasMissingTools(String),
    // TODO: use &str
    MissingBinary(String),
    MissingLanguageDefinition(std::path::PathBuf, String),
    MissingInput,
    ReadStdinError(std::io::Error),
    SerializeConfig(serde_json::Error),
    StdinWriteError,
    ToolError(String),
}

impl core::error::Error for MdsfError {}

impl core::fmt::Display for MdsfError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CheckModeChanges(file_count) => write!(
                f,
                "Found changes while running in check mode ({file_count} {})",
                if *file_count == 1 { "file" } else { "files" }
            ),
            Self::ConfigAlreadyExist => write!(f, "A config already exists in this directory"),
            Self::ConfigNotFound(path) => write!(f, "No config found at: '{}'", path.display()),
            Self::ConfigParse((path, error)) => {
                write!(
                    f,
                    "Error parsing config found at '{}' - {error}",
                    path.display()
                )
            }
            Self::Io(e) => e.fmt(f),
            Self::LanguageAliasClash(language, alias, already_set_by) => {
                write!(
                    f,
                    "'{language}' cannot be aliases to '{alias}' since it is already an alias of '{already_set_by}'"
                )
            }
            Self::LanguageAliasLanguagesContainsLanguage(language) => write!(
                f,
                "'{language}' cannot be used with an alias since it already has tools specified"
            ),
            Self::LanguageAliasMissingTools(alias) => write!(
                f,
                "'{alias}' cannot be used as an alias since it has no tools specified"
            ),
            Self::MissingBinary(binary_name) => write!(f, "{binary_name} not found in path"),
            Self::MissingLanguageDefinition(path, language) => {
                write!(f, "{} no tool configured for '{language}'", path.display())
            }
            Self::MissingInput => write!(f, "No input was provided to mdsf"),
            Self::ReadStdinError(error) => write!(f, "Error reading from stdin: {error}"),
            Self::SerializeConfig(e) => write!(f, "Error serializing config: {e}"),
            Self::StdinWriteError => write!(f, "Error writing to stdin"),
            Self::ToolError(stderr) => {
                let trimmed_stderr = stderr.trim();

                if trimmed_stderr.is_empty() {
                    write!(f, "Error running tool on codeblock")
                } else {
                    write!(f, "Error running tool on codeblock\n{trimmed_stderr}")
                }
            }
        }
    }
}

impl From<std::io::Error> for MdsfError {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
