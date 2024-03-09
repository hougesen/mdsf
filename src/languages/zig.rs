use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::zigfmt::format_using_zigfmt};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum ZigFormatter {
    #[default]
    #[serde(rename = "zigfmt")]
    ZigFmt,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Zig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: ZigFormatter,
}

impl Default for Zig {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: ZigFormatter::default(),
        }
    }
}

impl LanguageFormatter for Zig {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            ZigFormatter::ZigFmt => format_using_zigfmt(snippet_path).map(|res| res.1),
        }
    }
}
