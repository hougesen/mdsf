use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{djlint::format_using_djlint, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Mustache {
    #[default]
    #[serde(rename = "djlint")]
    DjLint,
}

impl Default for Lang<Mustache> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Mustache>::default(),
        }
    }
}

impl Default for MdsfFormatter<Mustache> {
    #[inline]
    fn default() -> Self {
        Self::Single(Mustache::DjLint)
    }
}

impl LanguageFormatter for Mustache {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::DjLint => format_using_djlint(snippet_path),
        }
    }
}

impl core::fmt::Display for Mustache {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DjLint => write!(f, "djlint"),
        }
    }
}

#[cfg(test)]
mod test_mustache {
    use super::Mustache;
    use crate::languages::Lang;

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Mustache>::default().enabled);
    }
}
