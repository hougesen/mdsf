use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{bicep_format::format_using_bicep_format, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Bicep {
    #[default]
    #[serde(rename = "bicep_format")]
    BicepFormat,
}

impl Default for Lang<Bicep> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Bicep>::default(),
        }
    }
}

impl Default for MdsfFormatter<Bicep> {
    #[inline]
    fn default() -> Self {
        Self::Single(Bicep::BicepFormat)
    }
}

impl LanguageFormatter for Bicep {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::BicepFormat => format_using_bicep_format(snippet_path),
        }
    }
}

impl core::fmt::Display for Bicep {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BicepFormat => write!(f, "bicep_format"),
        }
    }
}

#[cfg(test)]
mod test_bicep {
    use super::Bicep;
    use crate::languages::Lang;

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Bicep>::default().enabled);
    }
}
