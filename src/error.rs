#[derive(Debug)]
pub enum MdsfError {
    Io(std::io::Error),
    Fmt(core::fmt::Error),
    // TODO: use &std::path::Path
    ConfigParse(std::path::PathBuf),
    // TODO: use &std::path::Path
    FileNotFound(std::path::PathBuf),
    FormatterError,
    // TODO: use &str
    MissingBinary(String),
}

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
            Self::FormatterError => write!(f, "Error formatting codeblock"),
            Self::MissingBinary(binary_name) => write!(f, "{binary_name} was not found in path"),
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
