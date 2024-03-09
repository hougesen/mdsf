use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::dart_format::format_using_dart_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum DartFormatter {
    #[default]
    #[serde(rename = "dart_format")]
    DartFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Dart {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: DartFormatter,
}

impl Default for Dart {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: DartFormatter::default(),
        }
    }
}

impl LanguageFormatter for Dart {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            DartFormatter::DartFormat => format_using_dart_format(snippet_path).map(|res| res.1),
        }
    }
}
