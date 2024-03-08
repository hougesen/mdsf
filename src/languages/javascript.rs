use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::biome::format_using_biome};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Deserialize, JsonSchema)]
pub enum JavaScriptFormatter {
    #[default]
    Biome,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct JavaScript {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub formatter: JavaScriptFormatter,
}

impl Default for JavaScript {
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: JavaScriptFormatter::default(),
        }
    }
}

impl LanguageFormatter for JavaScript {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            JavaScriptFormatter::Biome => format_using_biome(snippet_path),
        }
    }
}
