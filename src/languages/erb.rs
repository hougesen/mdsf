use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{
        erb_formatter::format_using_erb_formatter, htmlbeautifier::format_using_htmlbeautifier,
        MdsfFormatter,
    },
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Erb {
    #[default]
    #[serde(rename = "erb-formatter")]
    ErbFormatter,
    #[serde(rename = "htmlbeautifier")]
    Htmlbeautifier,
}

impl Default for Lang<Erb> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Erb>::default(),
        }
    }
}

impl Default for MdsfFormatter<Erb> {
    #[inline]
    fn default() -> Self {
        Self::Single(Erb::ErbFormatter)
    }
}

impl LanguageFormatter for Erb {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::ErbFormatter => format_using_erb_formatter(snippet_path),
            Self::Htmlbeautifier => format_using_htmlbeautifier(snippet_path),
        }
    }
}

impl core::fmt::Display for Erb {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ErbFormatter => write!(f, "erb-formatter"),
            Self::Htmlbeautifier => write!(f, "htmlbeautifier "),
        }
    }
}

#[cfg(test)]
mod test_erb {
    use super::Erb;
    use crate::languages::Lang;

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Erb>::default().enabled);
    }
}
