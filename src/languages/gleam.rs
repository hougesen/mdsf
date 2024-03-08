use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::gleam_format::format_using_gleam_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum GleamFormatter {
    #[default]
    #[serde(rename = "gleam_format")]
    GleamFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Gleam {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: GleamFormatter,
}

impl Default for Gleam {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: GleamFormatter::default(),
        }
    }
}

impl LanguageFormatter for Gleam {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            GleamFormatter::GleamFormat => format_using_gleam_format(snippet_path),
        }
    }
}
