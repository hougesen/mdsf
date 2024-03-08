use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::biome::format_using_biome};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Deserialize, JsonSchema)]
pub enum TypeScriptFormatter {
    #[default]
    Biome,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct TypeScript {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub formatter: TypeScriptFormatter,
}

impl Default for TypeScript {
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: TypeScriptFormatter::default(),
        }
    }
}

impl LanguageFormatter for TypeScript {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            TypeScriptFormatter::Biome => format_using_biome(snippet_path),
        }
    }
}
