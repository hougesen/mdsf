use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{biome::format_using_biome, prettier::format_using_prettier},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum TypeScriptFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
    #[serde(rename = "biome")]
    Biome,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct TypeScript {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: TypeScriptFormatter,
}

impl Default for TypeScript {
    #[inline]
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
            TypeScriptFormatter::Prettier => format_using_prettier(snippet_path),
        }
    }
}
