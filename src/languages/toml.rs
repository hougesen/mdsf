use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::taplo::format_using_taplo};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TomlFormatter {
    #[default]
    #[serde(rename = "taplo")]
    Taplo,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Toml {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: TomlFormatter,
}

impl Default for Toml {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: TomlFormatter::default(),
        }
    }
}

impl LanguageFormatter for Toml {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            TomlFormatter::Taplo => format_using_taplo(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test_toml {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Toml, TomlFormatter};

    const INPUT: &str = "          package         =              \"mdsf\"
  author   = \"Mads Hougesen\"
  ";

    const EXTENSION: &str = crate::languages::Language::Toml.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        Toml::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Toml {
            enabled: false,
            formatter: TomlFormatter::Taplo,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l.format(snippet_path).expect("it to not fail").is_none());
    }

    #[test]
    fn test_taplo() {
        let l = Toml {
            enabled: true,
            formatter: TomlFormatter::Taplo,
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
