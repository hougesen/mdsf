use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::prettier::format_using_prettier};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum HtmlFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Html {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: HtmlFormatter,
}

impl Default for Html {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: HtmlFormatter::default(),
        }
    }
}

impl LanguageFormatter for Html {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            HtmlFormatter::Prettier => format_using_prettier(snippet_path, true).map(|res| res.1),
        }
    }
}
