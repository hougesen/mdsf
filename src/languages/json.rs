use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::biome::format_using_biome};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Deserialize, JsonSchema)]
pub enum JsonFormatter {
    #[default]
    Biome,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct Json {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub formatter: JsonFormatter,
}

impl Default for Json {
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
        }
    }
}
