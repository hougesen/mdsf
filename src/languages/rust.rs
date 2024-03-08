use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::rustfmt::format_using_rustfmt};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum RustFormatter {
    #[default]
    #[serde(rename = "rustfmt")]
    RustFmt,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Rust {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: RustFormatter,
}

impl Default for Rust {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: RustFormatter::default(),
        }
    }
}

impl LanguageFormatter for Rust {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            RustFormatter::RustFmt => format_using_rustfmt(snippet_path),
        }
    }
}
