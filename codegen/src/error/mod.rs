#[derive(Debug)]
pub enum CodegenError {
    Io(std::io::Error),
    FromUtf8(std::string::FromUtf8Error),
    SerdeJson(serde_json::Error),
    SerdeYaml(serde_yaml::Error),
    Reqwest(reqwest::Error),
}

impl std::error::Error for CodegenError {}

impl core::fmt::Display for CodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::FromUtf8(e) => e.fmt(f),
            Self::SerdeJson(e) => e.fmt(f),
            Self::SerdeYaml(e) => e.fmt(f),
            Self::Reqwest(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for CodegenError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::string::FromUtf8Error> for CodegenError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8(value)
    }
}

impl From<serde_json::Error> for CodegenError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

impl From<serde_yaml::Error> for CodegenError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::SerdeYaml(value)
    }
}

impl From<reqwest::Error> for CodegenError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}
