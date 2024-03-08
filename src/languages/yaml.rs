use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::prettier::format_using_prettier};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum YamlFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Yaml,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Yaml {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: YamlFormatter,
}

impl Default for Yaml {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: YamlFormatter::default(),
        }
    }
}

impl LanguageFormatter for Yaml {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            YamlFormatter::Yaml => format_using_prettier(snippet_path, true),
        }
    }
}
