#[derive(Debug)]
pub enum MdsfError {
    Io(std::io::Error),
    Fmt(core::fmt::Error),
    // TODO: use &std::path::Path
    ConfigParse(std::path::PathBuf),
    // TODO: use &std::path::Path
    FileNotFound(std::path::PathBuf),
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
}

impl std::error::Error for MdsfError {}

impl core::fmt::Display for MdsfError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Fmt(e) => e.fmt(f),
            Self::ConfigParse(path) => {
                write!(f, "Error parsing config found at '{}'", path.display())
            }
            Self::FileNotFound(path) => write!(
                f,
                "No file or directory with the name '{}' found",
                path.display()
            ),
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
                write!(f, "'{language}' cannot be aliases to '{alias}' since it is already an alias of '{already_set_by}'")
            }
            Self::LanguageAliasMissingTools(alias) => write!(
                f,
                "'{alias}' cannot be used as an alias since it has no tools specified"
            ),
            Self::MissingInput => write!(f, "No input was provided to mdsf"),
        }
    }
}

impl From<std::io::Error> for MdsfError {
    #[inline]
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<core::fmt::Error> for MdsfError {
    #[inline]
    fn from(value: core::fmt::Error) -> Self {
        Self::Fmt(value)
    }
}
