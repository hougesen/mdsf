use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::rubocop::format_using_rubocop};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RubyFormatter {
    #[default]
    #[serde(rename = "rubocop")]
    RuboCop,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Ruby {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: RubyFormatter,
}

impl Default for Ruby {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: RubyFormatter::default(),
        }
    }
}

impl LanguageFormatter for Ruby {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            RubyFormatter::RuboCop => format_using_rubocop(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test_ruby {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Ruby, RubyFormatter};

    const INPUT: &str =
        "def   add(  a ,                                                          b )
                        return a + b
                end";

    const EXTENSION: &str = crate::languages::Language::Ruby.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        Ruby::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Ruby {
            enabled: false,
            formatter: RubyFormatter::RuboCop,
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }
}
