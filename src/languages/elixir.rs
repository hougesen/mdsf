use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::mix_format::format_using_mix_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum ElixirFormatter {
    #[default]
    #[serde(rename = "mix_format")]
    MixFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Elixir {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: ElixirFormatter,
}

impl Default for Elixir {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: ElixirFormatter::default(),
        }
    }
}

impl LanguageFormatter for Elixir {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            ElixirFormatter::MixFormat => format_using_mix_format(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        formatters::setup_snippet,
        languages::{Language, LanguageFormatter},
    };

    use super::{Elixir, ElixirFormatter};

    const INPUT: &str = "";

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet =
            setup_snippet(INPUT, Language::Elixir.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        Elixir::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet =
            setup_snippet(INPUT, Language::Elixir.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Elixir {
            enabled: false,
            formatter: ElixirFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }
}
