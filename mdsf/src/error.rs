#[derive(Debug)]
pub enum MdsfError {
    Io(std::io::Error),
    // TODO: use &std::path::Path
    ConfigParse(std::path::PathBuf),
    FormatterError(String),
    // TODO: use &str
    MissingBinary(String),
    CheckModeChanges(u32),
    // TODO: rename 😅
    /// `languages` contains the language
    LanguageAliasLanguagesContainsLanguage(String),
    /// Another alias clashes
    LanguageAliasClash(String, String, String),
    LanguageAliasMissingTools(String),
    ReadStdinError(std::io::Error),
    MissingInput,
    StdinWriteError,
    SerializeConfig(serde_json::Error),
    ConfigAlreadyExist,
}

impl core::error::Error for MdsfError {}

impl core::fmt::Display for MdsfError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::ConfigParse(path) => {
                write!(f, "Error parsing config found at '{}'", path.display())
            }
            Self::ReadStdinError(error) => write!(f, "Error reading from stdin: {error}"),
            Self::FormatterError(stderr) => {
                let trimmed_stderr = stderr.trim();

                if trimmed_stderr.is_empty() {
                    write!(f, "Error formatting codeblock")
                } else {
                    write!(f, "Error formatting codeblock\n{trimmed_stderr}")
                }
            }
            Self::MissingBinary(binary_name) => write!(f, "{binary_name} was not found in path"),
            Self::CheckModeChanges(file_count) => {
                let file_or_files = if file_count == &1 { "file" } else { "files" };

                write!(
                    f,
                    "Found changes while running in check mode ({file_count} {file_or_files})"
                )
            }
            Self::LanguageAliasLanguagesContainsLanguage(language) => write!(
                f,
                "'{language}' cannot be used with an alias since it already has tools specified"
            ),
            Self::LanguageAliasClash(language, alias, already_set_by) => {
                write!(
                    f,
                    "'{language}' cannot be aliases to '{alias}' since it is already an alias of '{already_set_by}'"
                )
            }
            Self::LanguageAliasMissingTools(alias) => write!(
                f,
                "'{alias}' cannot be used as an alias since it has no tools specified"
            ),
            Self::MissingInput => write!(f, "No input was provided to mdsf"),
            Self::StdinWriteError => write!(f, "Error writing to stdin"),
            Self::SerializeConfig(e) => write!(f, "Error serializing config: {e}"),
            Self::ConfigAlreadyExist => write!(f, "A config already exists in this directory"),
        }
    }
}

impl From<std::io::Error> for MdsfError {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
