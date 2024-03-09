use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::shfmt::format_using_shfmt};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum BashFormatter {
    #[default]
    #[serde(rename = "shfmt")]
    Shfmt,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Bash {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: BashFormatter,
}

impl Default for Bash {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: BashFormatter::default(),
        }
    }
}

impl LanguageFormatter for Bash {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            BashFormatter::Shfmt => format_using_shfmt(snippet_path),
        }
    }
}
