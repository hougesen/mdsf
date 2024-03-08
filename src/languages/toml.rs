use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::taplo::format_using_taplo};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Deserialize, JsonSchema)]
pub enum TomlFormatter {
    #[default]
    Taplo,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct Toml {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub formatter: TomlFormatter,
}

impl Default for Toml {
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
            TomlFormatter::Taplo => format_using_taplo(snippet_path),
        }
    }
}
