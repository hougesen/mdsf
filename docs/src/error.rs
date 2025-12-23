#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Liquid(liquid::Error),
    SerdeJson(serde_json::Error),
}

impl core::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Liquid(e) => e.fmt(f),
            Self::SerdeJson(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<liquid::Error> for Error {
    fn from(value: liquid::Error) -> Self {
        Self::Liquid(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}
