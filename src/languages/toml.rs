use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{taplo::format_using_taplo, MdsfFormatter};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Toml {
    #[default]
    #[serde(rename = "taplo")]
    Taplo,
}

impl Default for Lang<Toml> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Toml>::default(),
        }
    }
}

impl Default for MdsfFormatter<Toml> {
    #[inline]
    fn default() -> Self {
        Self::Single(Toml::Taplo)
    }
}

impl LanguageFormatter for Toml {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Taplo => format_using_taplo(snippet_path),
        }
    }
}

impl core::fmt::Display for Toml {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Taplo => write!(f, "taplo"),
        }
    }
}

#[cfg(test)]
mod test_toml {
    use super::Toml;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "          package         =              \"mdsf\"
  author   = \"Mads Hougesen\"
  ";

    const EXTENSION: &str = crate::languages::Language::Toml.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Toml>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Lang::<Toml> {
            enabled: false,
            formatter: MdsfFormatter::Single(Toml::Taplo),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .is_none());
    }

    #[test]
    fn test_taplo() {
        let l = Lang::<Toml> {
            enabled: true,
            formatter: MdsfFormatter::Single(Toml::Taplo),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "package = \"mdsf\"
author = \"Mads Hougesen\"
";

        assert_eq!(output, expected_output);
    }
}
