use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::gofmt::format_using_gofmt};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum GoFormatter {
    #[default]
    #[serde(rename = "gofmt")]
    GoFmt,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Go {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: GoFormatter,
}

impl Default for Go {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: GoFormatter::default(),
        }
    }
}

impl LanguageFormatter for Go {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            GoFormatter::GoFmt => format_using_gofmt(snippet_path).map(|res| res.1),
        }
    }
}
