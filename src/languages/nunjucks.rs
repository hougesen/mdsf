use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{djlint::format_using_djlint, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Nunjucks {
    #[default]
    #[serde(rename = "djlint")]
    DjLint,
}

impl Default for Lang<Nunjucks> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Nunjucks>::default(),
        }
    }
}

impl Default for MdsfFormatter<Nunjucks> {
    #[inline]
    fn default() -> Self {
        Self::Single(Nunjucks::DjLint)
    }
}

impl LanguageFormatter for Nunjucks {
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

impl core::fmt::Display for Nunjucks {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DjLint => write!(f, "djlint"),
        }
    }
}

#[cfg(test)]
mod test_nunjucks {
    use super::Nunjucks;
    use crate::languages::Lang;

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Nunjucks>::default().enabled);
    }
}
