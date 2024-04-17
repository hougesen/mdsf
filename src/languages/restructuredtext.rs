use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{rstfmt::format_using_rstfmt, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum ReStructuredText {
    #[default]
    #[serde(rename = "rstfmt")]
    RstFmt,
}

impl Default for Lang<ReStructuredText> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<ReStructuredText>::default(),
        }
    }
}

impl Default for MdsfFormatter<ReStructuredText> {
    #[inline]
    fn default() -> Self {
        Self::Single(ReStructuredText::RstFmt)
    }
}

impl LanguageFormatter for ReStructuredText {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::RstFmt => format_using_rstfmt(snippet_path),
        }
    }
}

impl core::fmt::Display for ReStructuredText {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RstFmt => write!(f, "rstfmt"),
        }
    }
}
