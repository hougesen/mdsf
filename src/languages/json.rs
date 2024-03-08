use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{biome::format_using_biome, prettier::format_using_prettier},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum JsonFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
    #[serde(rename = "biome")]
    Biome,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Json {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: JsonFormatter,
}

impl Default for Json {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: JsonFormatter::default(),
        }
    }
}

impl LanguageFormatter for Json {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            JsonFormatter::Biome => format_using_biome(snippet_path),
            JsonFormatter::Prettier => format_using_prettier(snippet_path),
        }
    }
}
