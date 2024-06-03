use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{fnlfmt::format_using_fnlfmt, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Fennel {
    #[default]
    #[serde(rename = "fnlfmt")]
    Fnlfmt,
}

impl Default for Lang<Fennel> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Fennel>::default(),
        }
    }
}

impl Default for MdsfFormatter<Fennel> {
    #[inline]
    fn default() -> Self {
        Self::Single(Fennel::Fnlfmt)
    }
}

impl LanguageFormatter for Fennel {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Fnlfmt => format_using_fnlfmt(snippet_path),
        }
    }
}

impl core::fmt::Display for Fennel {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Fnlfmt => write!(f, "fnlfmt"),
        }
    }
}

#[cfg(test)]
mod test_fennel {
    use super::Fennel;
    use crate::languages::Lang;

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Fennel>::default().enabled);
    }
}
