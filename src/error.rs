#[derive(Debug)]
pub enum MdsfError {
    Io(std::io::Error),
    Fmt(core::fmt::Error),
}

impl core::fmt::Display for MdsfError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Fmt(e) => e.fmt(f),
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
