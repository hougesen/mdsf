use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{gdformat::format_using_gdformat, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum GDScript {
    #[default]
    #[serde(rename = "gdformat")]
    Gdformat,
}

impl Default for Lang<GDScript> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<GDScript>::default(),
        }
    }
}

impl Default for MdsfFormatter<GDScript> {
    #[inline]
    fn default() -> Self {
        Self::Single(GDScript::Gdformat)
    }
}

impl LanguageFormatter for GDScript {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Gdformat => format_using_gdformat(snippet_path),
        }
    }
}

impl core::fmt::Display for GDScript {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Gdformat => write!(f, "gdformat"),
        }
    }
}

#[cfg(test)]
mod test_gdscript {
    use super::GDScript;
    use crate::languages::Lang;

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<GDScript>::default().enabled);
    }
}
