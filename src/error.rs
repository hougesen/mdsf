#[derive(Debug)]
pub enum MdsfError {
    Io(std::io::Error),
    Fmt(core::fmt::Error),
    ConfigParse(std::path::PathBuf),
    FileNotFound(std::path::PathBuf),
}

impl core::fmt::Display for MdsfError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Fmt(e) => e.fmt(f),
            Self::ConfigParse(path) => f.write_fmt(format_args!(
                "Error parsing config found at '{}'",
                path.display()
            )),
            Self::FileNotFound(path) => f.write_fmt(format_args!(
                "No file or directory with the name '{}' found",
                path.display()
            )),
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
