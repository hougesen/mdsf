use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{puppet_lint::format_using_puppet_lint, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Puppet {
    #[default]
    #[serde(rename = "puppet-lint")]
    PuppetLint,
}

impl Default for Lang<Puppet> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Puppet>::default(),
        }
    }
}

impl Default for MdsfFormatter<Puppet> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![Self::Single(Puppet::PuppetLint)])])
    }
}

impl LanguageFormatter for Puppet {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::PuppetLint => format_using_puppet_lint(snippet_path),
        }
    }
}

impl core::fmt::Display for Puppet {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PuppetLint => write!(f, "puppet-lint"),
        }
    }
}

#[cfg(test)]
mod test_fortran {
    use super::Puppet;
    use crate::languages::Lang;

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Puppet>::default().enabled);
    }
}
