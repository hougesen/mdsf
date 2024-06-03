use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{fish_indent::format_using_fish_indent, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Fish {
    #[default]
    #[serde(rename = "fish_indent")]
    FishIndent,
}

impl Default for Lang<Fish> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Fish>::default(),
        }
    }
}

impl Default for MdsfFormatter<Fish> {
    #[inline]
    fn default() -> Self {
        Self::Single(Fish::FishIndent)
    }
}

impl LanguageFormatter for Fish {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::FishIndent => format_using_fish_indent(snippet_path),
        }
    }
}

impl core::fmt::Display for Fish {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FishIndent => write!(f, "fish_indent"),
        }
    }
}

#[cfg(test)]
mod test_fish {
    use super::Fish;
    use crate::languages::Lang;

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Fish>::default().enabled);
    }
}
