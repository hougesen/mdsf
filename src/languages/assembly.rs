use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{asmfmt::format_using_asmfmt, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Assembly {
    #[default]
    #[serde(rename = "asmfmt")]
    Asmfmt,
}

impl Default for Lang<Assembly> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Assembly>::default(),
        }
    }
}

impl Default for MdsfFormatter<Assembly> {
    #[inline]
    fn default() -> Self {
        Self::Single(Assembly::Asmfmt)
    }
}

impl LanguageFormatter for Assembly {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Asmfmt => format_using_asmfmt(snippet_path),
        }
    }
}

impl core::fmt::Display for Assembly {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Asmfmt => write!(f, "asmfmt"),
        }
    }
}

#[cfg(test)]
mod test_assembly {
    use super::Assembly;
    use crate::languages::Lang;

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Assembly>::default().enabled);
    }
}
