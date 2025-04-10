#[derive(Debug)]
pub enum MdsfError {
    CheckModeChanges(u32),
    ConfigAlreadyExist,
    ConfigNotFound(std::path::PathBuf),
    // TODO: use &std::path::Path
    ConfigParse(std::path::PathBuf),
    FormatterError(String),
    Io(std::io::Error),
    /// Another alias clashes
    LanguageAliasClash(String, String, String),
    // TODO: rename 😅
    /// `languages` contains the language
    LanguageAliasLanguagesContainsLanguage(String),
    LanguageAliasMissingTools(String),
    // TODO: use &str
    MissingBinary(String),
    MissingInput,
    ReadStdinError(std::io::Error),
    SerializeConfig(serde_json::Error),
    StdinWriteError,
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
            Self::ConfigParse(path) => {
                write!(f, "Error parsing config found at '{}'", path.display())
            }
            Self::FormatterError(stderr) => {
                let trimmed_stderr = stderr.trim();

                if trimmed_stderr.is_empty() {
                    write!(f, "Error formatting codeblock")
                } else {
                    write!(f, "Error formatting codeblock\n{trimmed_stderr}")
                }
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
            Self::MissingBinary(binary_name) => write!(f, "{binary_name} was not found in path"),
            Self::MissingInput => write!(f, "No input was provided to mdsf"),
            Self::ReadStdinError(error) => write!(f, "Error reading from stdin: {error}"),
            Self::SerializeConfig(e) => write!(f, "Error serializing config: {e}"),
            Self::StdinWriteError => write!(f, "Error writing to stdin"),
        }
    }
}

impl From<std::io::Error> for MdsfError {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
