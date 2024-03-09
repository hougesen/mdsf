use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::prettier::format_using_prettier};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum CssFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Css {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: CssFormatter,
}

impl Default for Css {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: CssFormatter::default(),
        }
    }
}

impl LanguageFormatter for Css {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            CssFormatter::Prettier => format_using_prettier(snippet_path, true).map(|res| res.1),
        }
    }
}
