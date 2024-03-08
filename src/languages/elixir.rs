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
            ElixirFormatter::MixFormat => format_using_mix_format(snippet_path),
        }
    }
}
