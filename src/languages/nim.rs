use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::nimpretty::format_using_nimpretty};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub enum NimFormatter {
    #[default]
    #[serde(rename = "nimpretty")]
    Nimpretty,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Nim {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: NimFormatter,
}

impl Default for Nim {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: NimFormatter::default(),
        }
    }
}

impl LanguageFormatter for Nim {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            NimFormatter::Nimpretty => format_using_nimpretty(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test_nim {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Nim, NimFormatter};

    const INPUT: &str = "proc           add( a         :int , b:int )        : int =
  return a +          b  ";

    const EXTENSION: &str = crate::languages::Language::Nim.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        Nim::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Nim {
            enabled: false,
            formatter: NimFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }
}
