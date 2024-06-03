use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{buildifier::format_using_buildifier, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Bazel {
    #[default]
    #[serde(rename = "buildifier")]
    Buildifier,
}

impl Default for Lang<Bazel> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Bazel>::default(),
        }
    }
}

impl Default for MdsfFormatter<Bazel> {
    #[inline]
    fn default() -> Self {
        Self::Single(Bazel::Buildifier)
    }
}

impl LanguageFormatter for Bazel {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Buildifier => format_using_buildifier(snippet_path),
        }
    }
}

impl core::fmt::Display for Bazel {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Buildifier => write!(f, "buildifier"),
        }
    }
}

#[cfg(test)]
mod test_bazel {
    use super::Bazel;
    use crate::languages::Lang;

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Bazel>::default().enabled);
    }
}
