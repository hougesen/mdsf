use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::nimpretty::format_using_nimpretty};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum NimFormatter {
    #[default]
    #[serde(rename = "nimpretty")]
    Nimpretty,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Nim {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: NimFormatter,
}

impl Default for Nim {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: NimFormatter::default(),
        }
    }
}

impl LanguageFormatter for Nim {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            NimFormatter::Nimpretty => format_using_nimpretty(snippet_path).map(|res| res.1),
        }
    }
}
