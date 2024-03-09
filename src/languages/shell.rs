use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::shfmt::format_using_shfmt};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum ShellFormatter {
    #[default]
    #[serde(rename = "shfmt")]
    Shfmt,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Shell {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: ShellFormatter,
}

impl Default for Shell {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: ShellFormatter::default(),
        }
    }
}

impl LanguageFormatter for Shell {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            ShellFormatter::Shfmt => format_using_shfmt(snippet_path).map(|res| res.1),
        }
    }
}
