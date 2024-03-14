use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::crystal_format::format_using_crystal_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum CrystalFormatter {
    #[default]
    #[serde(rename = "crystal_format")]
    CrystalFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Crystal {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: CrystalFormatter,
}

impl Default for Crystal {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: CrystalFormatter::default(),
        }
    }
}

impl LanguageFormatter for Crystal {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            CrystalFormatter::CrystalFormat => format_using_crystal_format(snippet_path),
        }
        .map(|res| res.1)
    }
}

#[cfg(test)]
mod test_crystal {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Crystal, CrystalFormatter};

    const INPUT: &str = "def add(a, b)  return a + b end";

    const EXTENSION: &str = crate::languages::Language::Crystal.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Crystal::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Crystal {
            enabled: false,
            formatter: CrystalFormatter::CrystalFormat,
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_crystal_format() {
        let expected_output = "def add(a, b)
  return a + b
end
";
        let l = Crystal {
            enabled: true,
            formatter: CrystalFormatter::CrystalFormat,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}
