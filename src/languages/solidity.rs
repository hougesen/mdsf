use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{forge_fmt::format_using_forge_fmt, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Solidity {
    #[default]
    #[serde(rename = "forge_fmt")]
    ForgeFmt,
}

impl Default for Lang<Solidity> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Solidity>::default(),
        }
    }
}

impl Default for MdsfFormatter<Solidity> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![Self::Single(Solidity::ForgeFmt)])])
    }
}

impl LanguageFormatter for Solidity {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::ForgeFmt => format_using_forge_fmt(snippet_path),
        }
    }
}

impl core::fmt::Display for Solidity {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ForgeFmt => write!(f, "forge_fmt"),
        }
    }
}

#[cfg(test)]
mod test_fortran {
    use super::Solidity;
    use crate::languages::Lang;

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Solidity>::default().enabled);
    }
}
