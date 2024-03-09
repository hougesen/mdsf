use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::taplo::format_using_taplo};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum TomlFormatter {
    #[default]
    #[serde(rename = "taplo")]
    Taplo,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Toml {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: TomlFormatter,
}

impl Default for Toml {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: TomlFormatter::default(),
        }
    }
}

impl LanguageFormatter for Toml {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            TomlFormatter::Taplo => format_using_taplo(snippet_path).map(|res| res.1),
        }
    }
}
