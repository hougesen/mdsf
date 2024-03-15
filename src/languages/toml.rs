use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{format_multiple, taplo::format_using_taplo, MdsfFormatter},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum TomlFormatter {
    #[default]
    #[serde(rename = "taplo")]
    Taplo,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Toml {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<TomlFormatter>,
}

impl Default for Toml {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<TomlFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<TomlFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Single(TomlFormatter::Taplo)
    }
}

impl LanguageFormatter<TomlFormatter> for Toml {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        format_multiple(&self.formatter, snippet_path, &Self::format_single)
            .map(|(_should_continue, output)| output)
    }

    #[inline]
    fn format_single(
        formatter: &TomlFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            TomlFormatter::Taplo => format_using_taplo(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_toml {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

    use super::{Toml, TomlFormatter};

    const INPUT: &str = "          package         =              \"mdsf\"
  author   = \"Mads Hougesen\"
  ";

    const EXTENSION: &str = crate::languages::Language::Toml.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Toml::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Toml {
            enabled: false,
            formatter: MdsfFormatter::Single(TomlFormatter::Taplo),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l.format(snippet_path).expect("it to not fail").is_none());
    }

    #[test]
    fn test_taplo() {
        let l = Toml {
            enabled: true,
            formatter: MdsfFormatter::Single(TomlFormatter::Taplo),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "package = \"mdsf\"
author = \"Mads Hougesen\"
";

        assert_eq!(output, expected_output);
    }
}
